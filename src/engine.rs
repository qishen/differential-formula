use std::{borrow::Cow, os::linux::raw};
use std::convert::*;
use std::sync::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{BTreeMap, HashSet, HashMap};

use bimap::*;
use indexmap::IndexMap;
use timely::{dataflow::*, logging::ProgressEventTimestamp};
use differential_dataflow::*;
use differential_dataflow::input::InputSession;
use differential_dataflow::operators::join::*;
use differential_dataflow::operators::*;
use differential_dataflow::hashable::*;

// Import from local ddlog repo and APIs are subject to constant changes
use differential_datalog::ddval::*;
use differential_datalog::record::*;
use differential_datalog::program::{Arrangement, CachingMode, ProgNode, Program, RecursiveRelation, Relation, Rule as DDRule, RunningProgram, XFormArrangement, XFormCollection};
// use ddlog_lib::*;

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::module::*;
use crate::rule::*;
use crate::parser::combinator::*;

/**
1. Formula Composite types translate to `Relation` in ddlog
2. Formula terms translate to `DDValue` that can run in the dataflow. How about `Record`?
3. Translate conjunction of predicates. What's the best way to do optimal joins? 
    Pred1(x1, x2,...,xn, y1, y2,..., yn), Pred2(y1, y2,..., yn, z1, z2,...,zn)
    Create two arrangements 
        1. ((y1,..,yn), (x1,..,xn)) arranged by (y1,..,yn) into ((y1,..yn), Pred1)
        2. ((y1,..,yn), (z1,..,zn)) arranged by (y1,..,yn) into ((y1,..yn), Pred2)

Convert from 3D to 2D and generate a new relation.
Z(A(a), B(b), U(m, x, y)) :- X(M(N(a), N(b)), D(m), Y(n, m)), Y(P(a), Q(P(b, m, y)), x).
T1(a, b, m, n) :- X(M(N(a), N(b)), D(m), Y(n, m)).
Flatten out: X(v1, v2, v3), v1 = M(v4, v5), v2 = D(m), v3 = Y(n, m), v4 = N(a), v5 = N(b).
T2(a, b, m, x, y) :- Y(P(a), Q(J(b, m, y)), x)
(key: (a, b, m), left: (n), right: (x, y))

DDValue -> (DDValue, DDValue)
X(M(N(a), N(b)), D(m), Y(n, m)) -> ((a, b, m), (n))
Y(P(a), Q(P(b, m, y)), x) -> ((a, b, m), (x, y))
**/

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FormulaArrKey {
    normalized_term: AtomicTerm,
    normalized_key: Vec<AtomicTerm>,
    normalized_val: Vec<AtomicTerm>,
    // The position of current arrangement in ddlog rules, the first number is 
    // the relation id and the second number is the index of arrangements in that relation.
    // position: (usize, usize)
}

impl FormulaArrKey {
    fn new(term: AtomicTerm, key: &Vec<AtomicTerm>, val: &Vec<AtomicTerm>) -> Result<Self, String> {
        let vars = term.variables();
        let (normalized_term, vmap) = term.normalize();
        // If `key` is not provided, use all variables in the term as the key and `val` is empty.
        if key.len() == 0 {
            let mut normalized_key: Vec<AtomicTerm> = vars.iter().map(|x| { 
                vmap.get(x).unwrap().clone()
            }).collect();
            normalized_key.sort();
            let arr_key = FormulaArrKey {
                normalized_term,
                normalized_key,
                normalized_val: vec![]
            };
            return Ok(arr_key);
        } else {
            // Make sure both `key` and `val` are subsets of the variable terms in term. 
            if !key.iter().all(|x| { vars.contains(x) }) { 
                return Err(format!("`key` {:?} does not exist in the variables of term {}.", key, term)); 
            } else if !val.iter().all(|x| { vars.contains(x) }) {
                return Err(format!("`val` {:?} does not exist in the variables of term {}.", val, term)); 
            }

            let mut normalized_key: Vec<AtomicTerm> = key.iter().map(|x| 
                vmap.get(x).unwrap().clone()
            ).collect();
            normalized_key.sort();

            let mut normalized_val: Vec<AtomicTerm> = val.iter().map(|x|
                vmap.get(x).unwrap().clone()
            ).collect();
            normalized_val.sort();

            let arr_key = FormulaArrKey { 
                normalized_term, 
                normalized_key,
                normalized_val,
            };

            return Ok(arr_key);
        }
    }

    /// The `FormulaArrKey` arr1 can be reused if another `FormulaArrKey` arr2 has the same normalized term,
    /// same normalized key and arr2's normalized val is a subset of the normalized val of arr1. 
    fn is_subset(&self, other: &Self) -> bool {
        let val_set: HashSet<&AtomicTerm> = self.normalized_val.iter().collect();
        let other_val_set: HashSet<&AtomicTerm> = other.normalized_val.iter().collect();
        self.normalized_term == other.normalized_term && 
        self.normalized_key == other.normalized_key &&
        val_set.is_subset(&other_val_set) 
    }

    /// Check if a term can be matched to `FormulaArrKey` after normalization.
    fn is_matched(&self, term: &AtomicTerm) -> bool {
        let (normalized_term, _) = term.normalize();
        normalized_term == self.normalized_term
    }
}

struct FormulaTermIndex {
    // Use `IndexMap` because the relation map needs to be iterated in 
    // the same order as the `Relation` is inserted in the map.
    relation_map: IndexMap<String, Relation>,
    // We need to look up by both relation id and relation name so both
    // of them are indexed.
    relation_id_map: BiMap<usize, String>,
    // Each Arrangement is represented by a `FormulaArrKey` that contains
    // a normalized term and a list of terms as the key.
    arr_map: HashMap<FormulaArrKey, Arrangement>,
    // Both arrid in ddlog and `FormulaArrKey` need to be indexed. 
    arr_id_map: BiMap<(usize, usize), FormulaArrKey>
}

impl FormulaTermIndex {
    fn new(relations: Vec<(String, Relation)>) -> Self {
        let mut relation_map: IndexMap<String, Relation> = IndexMap::new();
        let mut relation_id_map: BiMap<usize, String> = BiMap::new();

        for (name, relation) in relations.into_iter() {
            let rid = relation.id.clone();
            relation_map.insert(name.clone(), relation);
            relation_id_map.insert(rid, name.clone());
        }

        FormulaTermIndex {
            relation_map,
            relation_id_map,
            arr_map: HashMap::new(),
            arr_id_map: BiMap::new()
        }
    }

    fn input_relations(&self) -> Vec<String> {
        let mut relnames = vec![];
        for (name, _) in self.relation_map.iter() {
            let input_relname = format!("{}_input", name);
            if self.relation_map.contains_key(&input_relname) {
                relnames.push(input_relname);
            }
        }
        relnames
    }

    fn noninput_relations(&self) -> Vec<String> {
        let mut relnames = vec![];
        for (name, _) in self.relation_map.iter() {
            let input_relname = format!("{}_input", name);
            if self.relation_map.contains_key(&input_relname) {
                relnames.push(name.clone());
            }
        }
        relnames
    }

    fn relation_by_id(&self, id: usize) -> Option<&Relation> {
        if let Some(relation_name) = self.relation_id_map.get_by_left(&id) {
            self.relation_map.get(relation_name)
        } else { None }
    }

    fn relation_by_name(&self, name: &str) -> Option<&Relation> {
        self.relation_map.get(name)
    }

    fn rid_by_name(&self, name: &str) -> Option<usize> {
        self.relation_id_map.get_by_right(name).map(|x| x.clone())
    }

    fn arrangement_by_id(&self, arr_id: (usize, usize)) -> Option<&Arrangement> {
        if let Some(arr_key) = self.arr_id_map.get_by_left(&arr_id) {
            self.arr_map.get(arr_key)
        } else { None }
    }

    fn arrangement_by_key(&self, arr_key: &FormulaArrKey) -> Option<&Arrangement> {
        self.arr_map.get(arr_key)
    }

    fn arrid_by_key(&self, arr_key: &FormulaArrKey) -> Option<(usize, usize)> {
        self.arr_id_map.get_by_right(arr_key).map(|x| x.clone())
    }

    /// Generate arrangement and add it to the corresponding relation.
    fn add_arrangement(&mut self, pred: &AtomicTerm, key: &Vec<AtomicTerm>, val: &Vec<AtomicTerm>) {
        // Two relations for same type `Type` and `Type_input` and the arrangement has to be added
        // to the input relation.
        // let input_relation_name = format!("{}_input", pred.type_id());
        // Since `Relation_input` and `Relation` are synced we do not have to build arrangement
        // upon input relation.
        let relation_name = pred.type_id().into_owned();
        let relation_id = self.rid_by_name(&relation_name).unwrap();
        let relation = self.relation_map.get_mut(&relation_name).unwrap();
        let arrkey = FormulaArrKey::new(pred.clone(), key, val).unwrap();
        let arr = Self::into_arrangment(pred, key, val);

        // Add new arrangement to the end of `arrangements` in this relation.
        let arr_id = (relation_id, relation.arrangements.len());
        relation.arrangements.push(arr.clone());

        self.arr_id_map.insert(arr_id, arrkey.clone());
        self.arr_map.insert(arrkey, arr);
    }

    /// Add a `DDRule` into the corresponding `Relation`.
    fn add_rule(&mut self, rid: usize, rule: DDRule) -> bool {
        if let Some(relation_name) = self.relation_id_map.get_by_left(&rid) {
            let relation = self.relation_map.get_mut(relation_name).unwrap();
            relation.rules.push(rule);
            true
        } else { false } 
    }

    /// Map from `Relation_input` to `Relation` because in Formula every relation is both
    /// both input and out relation in ddlog definition.
    fn create_input_map_rule(&self, relname: String) -> DDRule {
        let rid = self.rid_by_name(&format!("{}_input", relname)).unwrap();
        let rule_str = format!("{}(..) :- {}_input(..) ", relname, relname);
        let self_map_rule = DDRule::CollectionRule {
            description: Cow::from(rule_str.clone()),
            rel: rid,
            xform: Some(XFormCollection::Map {
                description: Cow::from(rule_str.clone()),
                mfun: Box::new(move |val: DDValue| -> DDValue { val }),
                next: Box::new(Some(XFormCollection::Inspect {
                    description: Cow::from("Inspect newly derived terms"),
                    ifun: Box::new(move |val, ts, weight| {
                        println!("From {} rule, val: {:?}, timestamp: {:?}, weight: {:?}", 
                            rule_str, val, ts, weight)
                    }),
                    next: Box::new(None)
                }))
            })
        };
        self_map_rule

    }

    /// Find self arrangement and derive new term in the head of the rule.
    fn create_map_rule(&self, matching_term: &AtomicTerm, head: Vec<&AtomicTerm>) -> DDRule {
        let map_rule_str = format!("{:?} :- {}", head, matching_term);
        let head_terms: Vec<AtomicTerm> = head.iter().map(|x| x.clone().clone()).collect();
        // If we want the head to have only one term.
        let head_term = head_terms.get(0).unwrap().clone();

        let type_name = matching_term.type_id();
        let rid = self.rid_by_name(&type_name).unwrap();
        let matching_term = matching_term.clone();
        let map_rule = DDRule::CollectionRule {
            description: Cow::from(format!("{:?} :- {:?} ", &head_terms, matching_term)),
            rel: rid,
            xform: Some(XFormCollection::FilterMap {
                description: Cow::from(map_rule_str.clone()),
                fmfun: Box::new(move |val: DDValue| -> Option<DDValue> { 
                    let term = <AtomicTerm>::from_ddvalue_ref(&val);
                    if let Some(m) = matching_term.match_to(term) {
                        // let derived_terms: Vec<AtomicTerm> = head_terms.iter().map(|x| 
                        //     x.propagate(&m)
                        // ).collect();
                        // Some(derived_terms.into_ddvalue())
                        let term = head_term.propagate(&m);
                        Some(term.into_ddvalue())
                    } else { None }
                }),
                // next: Box::new(Some(XFormCollection::Inspect {
                //     description: Cow::from("Inspect newly derived terms"),
                //     ifun: Box::new(move |val, ts, weight| {
                //         println!("From map rule {}, val: {:?}, timestamp: {:?}, weight: {:?}", 
                //         map_rule_str.clone(), val, ts, weight)
                //     }),
                //     next: Box::new(None)
                // }))
                next: Box::new(None)
                // next: Box::new(Some(XFormCollection::FlatMap {
                //     description: Cow::from(format!("Derived head terms {:?} from flat map", head)),
                //     fmfun: Box::new(|v: DDValue| {
                //         let head_terms = <Vec<AtomicTerm>>::from_ddvalue(v);
                //         Some(Box::new(head_terms.into_iter().map(|x| x.into_ddvalue() )))
                //     }),
                //     next: Box::new(Some(XFormCollection::Inspect {
                //         description: Cow::from("Inspect newly derived terms"),
                //         ifun: Box::new(move |val, ts, weight| {
                //             println!("From map rule {}, val: {:?}, timestamp: {:?}, weight: {:?}", 
                //             map_rule_str.clone(), val, ts, weight)
                //         }),
                //         next: Box::new(None)
                //     }))
                // }))
            })
        };
        map_rule
    }

    /// Find two arrangements by looking up `FormulaArrKey`, join two arrangements and derive new 
    /// terms in the head of the rule.
    /// TODO: Change to Worst Case Optimal Joins.
    fn create_join_rule(&mut self, t1: &AtomicTerm, t2: &AtomicTerm, head: Vec<&AtomicTerm>) -> DDRule {
        let (key, val1, val2) = t1.variable_diff(t2);
        let join_rule_str = format!("{:?} :- {}, {}", head, t1, t2);
        let head_terms: Vec<AtomicTerm> = head.iter().map(|x| x.clone().clone()).collect();
        // If we want the head to have only one term.
        let head_term = head_terms.get(0).unwrap().clone();

        let arr_key1 = FormulaArrKey::new(t1.clone(), &key, &val1).unwrap();
        let arr_key2 = FormulaArrKey::new(t2.clone(), &key, &val2).unwrap();
        let arr_id1 = self.arrid_by_key(&arr_key1).unwrap();
        let arr_id2 = self.arrid_by_key(&arr_key2).unwrap();

        let join_rule = DDRule::ArrangementRule {
            description: Cow::from(join_rule_str.clone()),
            arr: arr_id1,
            xform: XFormArrangement::Join {
                description: Cow::from(join_rule_str.clone()),
                ffun: None,
                arrangement: arr_id2,
                jfun: Box::new(move |k: &DDValue, v1: &DDValue, v2: &DDValue| -> Option<DDValue> {
                    let mut tmap = HashMap::new();
                    // Hmm, is it too expensive?
                    let key_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(k);
                    let val1_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(v1);
                    let val2_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(v2);

                    key.iter().zip(key_terms).for_each(|(k, v)| { tmap.insert(k, v); });
                    val1.iter().zip(val1_terms).for_each(|(k, v)| { tmap.insert(k, v); });
                    val2.iter().zip(val2_terms).for_each(|(k, v)| { tmap.insert(k, v); });

                    let result = head_term.propagate(&tmap).into_ddvalue();
                    // let new_terms: Vec<AtomicTerm> = head_terms.iter().map(|term| {
                    //     term.propagate(&tmap)
                    // }).collect();

                    // let result = new_terms.into_ddvalue(); 
                    Some(result)
                }),
                // next: Box::new(Some(XFormCollection::Inspect {
                //     description: Cow::from("Inspect newly derived terms"),
                //     ifun: Box::new(move |val, ts, weight| {
                //         println!("From Join rule {}, val: {:?}, timestamp: {:?}, weight: {:?}", 
                //             join_rule_str, val, ts, weight)
                //     }),
                //     next: Box::new(None)
                // }))
                next: Box::new(None)
                // next: Box::new(Some(XFormCollection::FlatMap {
                //     description: Cow::from(format!("Flat map derived head terms {:?}", head)),
                //     fmfun: Box::new(|v: DDValue| {
                //         let head_terms = <Vec<AtomicTerm>>::from_ddvalue(v);
                //         Some(Box::new(head_terms.into_iter().map(|x| x.into_ddvalue() )))
                //     }),
                //     next: Box::new(Some(XFormCollection::Inspect {
                //         description: Cow::from("Inspect newly derived terms"),
                //         ifun: Box::new(move |val, ts, weight| {
                //             println!("From Join rule {}, val: {:?}, timestamp: {:?}, weight: {:?}", 
                //                 join_rule_str, val, ts, weight)
                //         }),
                //         next: Box::new(None)
                //     }))
                // }))
            }
        };

        join_rule
    }

    /// `key_vars` and `val_vars` must be disjoint subset of the variables in `matching_term` 
    /// e.g. Edge(Node(a), b), Edge(c, b) 
    /// where key is variable `b` and the val is `a` or `c`. 
    /// TODO: Evalution with additional constraints that may generate temporary new terms in 
    /// the the execution of the body of a rule. Node(a) = { Node(1), Node(2) } then 
    /// Node(x) = { Node(2), Node(3) }. Let's check if the new terms are existent in the term
    /// database rathen than join operations followed by filter operation.
    /// Edge(Node(a), b), a' = Node(x), x = a + 1, Edge(c, b) 
    fn into_arrangment(matching_term: &AtomicTerm, key: &Vec<AtomicTerm>, val: &Vec<AtomicTerm>) -> Arrangement {
        let key_vars = key.clone();
        let val_vars = val.clone();
        let matching_term = matching_term.clone();
        let arr = Arrangement::Map {
            name: Cow::from(format!("Matching {} with key: {:?} and val: {:?}", 
                matching_term, 
                key_vars, 
                val_vars)
            ),
            afun: Box::new(move |v: DDValue| -> Option<(DDValue, DDValue)> {
                let term = <AtomicTerm>::from_ddvalue(v);
                if let Some(term_match) = matching_term.match_to(&term) {
                    let shared_match: Vec<AtomicTerm> = key_vars.iter().map(|x| { 
                        term_match.get(&x).unwrap().clone().clone() 
                    }).collect();

                    let left_match: Vec<AtomicTerm> = val_vars.clone().iter().map(|x| {
                        term_match.get(&x).unwrap().clone().clone()
                    }).collect();

                    let key = shared_match.into_ddvalue(); 
                    let val = left_match.into_ddvalue();
                    return Some((key, val))
                }
                return None;
            }),
            queryable: true,
        };

        arr
    }
}

#[derive(Debug)]
enum FormulaProgNode {
    Rel(String),
    SCC(Vec<String>)
}

struct FormulaProgram {
    // Indexed relations and arrangements 
    index: FormulaTermIndex,
    structure: Vec<FormulaProgNode>
}

impl FormulaProgram {
    pub fn new(index: FormulaTermIndex, nodes: Vec<FormulaProgNode>) -> Self {
        FormulaProgram {
            index,
            structure: nodes
        }
    } 

    /// Generate a DDLog program from FORMULA index and the program structure.
    pub fn generate(&self) -> Program {
        // The relations must be added in the right order otherwise may fail to load
        // arrangement from dependent relation.
        let mut nodes = vec![];
        println!("Print nodes: {:?}", self.structure);
        for node in self.structure.iter() {
            match node {
                FormulaProgNode::Rel(rel_name) => {
                    let rel = self.index.relation_by_name(rel_name).unwrap().clone(); 
                    let node = ProgNode::Rel { rel };
                    nodes.push(node);
                },
                FormulaProgNode::SCC(rel_names) => {
                    let recursive_rels: Vec<RecursiveRelation> = rel_names.iter().map(|relname| {
                        let rel = self.index.relation_by_name(relname).unwrap().clone();
                        RecursiveRelation { rel, distinct: true }
                    }).collect();
                    let scc_node = ProgNode::SCC { rels: recursive_rels };
                    nodes.push(scc_node);
                }
            };
        }

        let program: Program = Program {
            nodes,
            delayed_rels: vec![],
            init_data: vec![],
        };

        program
    }
}

struct DomainDataflow {
    // TODO: Each stratum should have different index.
    prog: FormulaProgram,
    running_program: Option<RunningProgram>,
}

impl DomainDataflow {
    fn is_running(&self) -> bool {
        self.running_program.is_some()
    }
    
    fn run(&mut self) {
        let program = self.prog.generate();
        self.running_program = Some(program.run(1).unwrap())
    }

    fn stop(&mut self) -> Result<(), String> {
        if self.is_running() {
            let running = self.running_program.as_mut().unwrap();
            running.stop()
        } else { Err("No running program exists.".to_string()) }
    }

    fn insert_terms(&mut self, terms: Vec<AtomicTerm>) {
        if !self.is_running() {
            self.run();
        }
        let running_program = self.running_program.as_mut().unwrap();
        running_program.transaction_start().unwrap();
        for term in terms {
            // Must add the new terms into the correct input relation based on term type.
            let type_name = format!("{}_input", term.type_id());
            let rid = self.prog.index.rid_by_name(&type_name).unwrap();
            println!("Inserting the term: {}", term);
            running_program.insert(rid, term.into_ddvalue()).unwrap();
        }
        running_program.transaction_commit().unwrap();
    }
}

struct FormulaExecEngine {
    env: Env,
    domains: HashMap<String, DomainDataflow>
}

impl FormulaExecEngine {
    fn dataflow_by_name(&mut self, name: &str) -> &mut DomainDataflow {
        self.domains.get_mut(name).unwrap()
    }

    fn new(env: Env) -> Self {
        let mut dataflows = HashMap::new();
        for (domain_name, domain) in env.domain_map.iter() {
            let meta = domain.meta_info();
            let mut relation_counter = 0;
            // `Relation`, Relation id and a unique string to represent relation.
            let mut sorted_relations = Vec::new();

            // Find all relations that need to be put into SCC.
            let mut scc_relname_set = HashSet::new();
            for rule in meta.rules() {
                let head = rule.head();
                for term in head {
                    scc_relname_set.insert(term.type_id().into_owned());
                }
            }

            // Relations that need to be put into a SCC.
            let mut scc_rels = vec![];
            // node could be a relation or a list of relations.
            let mut formula_nodes = vec![];

            // All relations in stratum 0 only for inputs and we may want to build some
            // arrangements for each relation.
            // type `Path` is represented by two relations that one only for input and 
            // the other for the following reasoning and derivation from input or previous stratums.
            for raw_type in meta.sorted_composite_types().into_iter() {
                let name = raw_type.type_id(); 
                let input_relation_id = relation_counter;
                let relation_id = input_relation_id + 1; 
                relation_counter += 2;

                let input_rel = Relation {
                    name: Cow::from(format!("Input relation {} in Stratum 0", name)),
                    input: true,
                    distinct: true,
                    caching_mode: CachingMode::Set,
                    key_func: None,
                    id: input_relation_id,
                    rules: vec![],
                    arrangements: vec![],
                    change_cb: None,
                };

                let rel = Relation {
                    name: Cow::from(format!("Relation {} in Stratum 1", name)),
                    input: false,
                    // Well, I thought it has to be distinct but ddlog doesn't set it to true.
                    distinct: false,
                    caching_mode: CachingMode::Set,
                    key_func: None,
                    id: relation_id,
                    rules: vec![],
                    arrangements: vec![],
                    change_cb: None,
                };

                println!("Id {} for input relation name: {}_input", input_relation_id, name);
                println!("Id {} for relation name: {}", relation_id, name);

                sorted_relations.push((format!("{}_input", name), input_rel));
                formula_nodes.push(FormulaProgNode::Rel(format!("{}_input", name)));
                sorted_relations.push((format!("{}", name), rel));
                // Put some relations into SCC for mutually recursive rules.
                if scc_relname_set.contains(name.as_ref()) {
                    scc_rels.push(name.into_owned());
                } else {
                    formula_nodes.push(FormulaProgNode::Rel(name.into_owned()));
                }
            }

            formula_nodes.push(FormulaProgNode::SCC(scc_rels));
            let mut index = FormulaTermIndex::new(sorted_relations);

            // Map every `Relation_input` to `Relation` because input relation cannot be used in recursive rules.
            for relname in index.noninput_relations() {
                let self_map_rule = index.create_input_map_rule(relname.clone());
                let rid = index.rid_by_name(&relname).unwrap();
                index.add_rule(rid, self_map_rule);
            }

            // TODO: Assume all rules are sorted and in the same Stratum 1 for now.
            // Assume all rules are mutually recursive to test loop.
            for rule in meta.rules() {
                println!("Create dataflow for rule {:?}", rule);
                let pred_terms: Vec<AtomicTerm> = rule.predicate_constraints().iter().filter_map(|constraint| {
                    match *constraint {
                        Constraint::Predicate(pred) => {
                            if !pred.negated { Some(pred.term.clone()) } else { None }
                        },
                        _ => None
                    }
                }).collect();

                let head = rule.head();
                
                if pred_terms.len() == 1 {
                    // Match to only one term predicate and derive new terms without join operation. 
                    let t1 = pred_terms.get(0).unwrap().clone();
                    let map_rule = index.create_map_rule(&t1, head.clone());
                    // Add rule to the relation of the head terms.
                    let h = head.get(0).unwrap().clone();
                    let rid = index.rid_by_name(&h.type_id()).unwrap();
                    index.add_rule(rid, map_rule);
                } else {
                    // Just pick the firt two preds for experiment for now.
                    let t1 = pred_terms.get(0).unwrap().clone();
                    let t2 = pred_terms.get(1).unwrap().clone();
                    let (normalized_t1, t1_map) = t1.normalize();
                    let (normalized_t2, t2_map) = t2.normalize();

                    println!("Normalization {}, {:?}", normalized_t1, t1_map);
                    println!("Normalization {}, {:?}", normalized_t2, t2_map);

                    let (shared_vars, left_vars, right_vars) = t1.variable_diff(&t2);
                    println!("shared vars: {:?}, left vars: {:?}, right vars: {:?}", 
                        shared_vars, left_vars, right_vars);

                    index.add_arrangement(&t1, &shared_vars, &left_vars);
                    index.add_arrangement(&t2, &shared_vars, &right_vars);
                    
                    println!("{:?}", index.arr_id_map); 

                    let join_rule = index.create_join_rule(&t1, &t2, head.clone());
                    let h = head.get(0).unwrap().clone();
                    let rid = index.rid_by_name(&h.type_id()).unwrap();
                    index.add_rule(rid, join_rule);
                }
            }
            
            // TODO: How about stratums after Stratum 1.
            let formula_program = FormulaProgram::new(index, formula_nodes);
            let dataflow = DomainDataflow {
                prog: formula_program,
                running_program: None
            };

            dataflows.insert(domain_name.clone(), dataflow);
        }

        FormulaExecEngine { env, domains: dataflows }
    }
}

mod tests {
    use super::*;
    use crate::parser::combinator::parse_program;
    use std::path::Path;
    use std::fs;
    use rand::*;

    #[test]
    fn test_parse_models() {
        let path = Path::new("./tests/testcase/p0.4ml");
        let content = fs::read_to_string(path).unwrap() + "EOF";
        let (_, program_ast) = parse_program(&content);
          
        let env: Env = program_ast.build_env();
        let graph = env.get_domain_by_name("Graph").unwrap();
        let m = env.get_model_by_name("m").unwrap();
        println!("{:#?}", graph);

        let terms: Vec<AtomicTerm> = m.terms().into_iter().map(|x| x.clone()).collect();

        let mut engine = FormulaExecEngine::new(env);
        let df = engine.dataflow_by_name("Graph");
        df.run();
        df.insert_terms(terms);

        // let rule0 = graph.meta_info().rules().get(0).unwrap();
        let node1_ast = term(&format!("Node(1)~")[..]).unwrap().1;
        let node1: AtomicTerm = node1_ast.into();
        let edge_ab_ast = term(&format!("Path(a, b)~")[..]).unwrap().1;
        let edge_ab: AtomicTerm = edge_ab_ast.into();
        let arr_key = FormulaArrKey::new(
            edge_ab, 
            &vec![AtomicTerm::gen_raw_variable_term(String::from("a"), vec![])], 
            &vec![AtomicTerm::gen_raw_variable_term(String::from("b"), vec![])], 
        ).unwrap();

        println!("arr_key: {:?}", arr_key);
        println!("Arrangement index: {:?}", df.prog.index.arr_id_map);

        let arrid = df.prog.index.arrid_by_key(&arr_key).unwrap();
        if let Some(ref mut program) = df.running_program {
            // let result = program.query_arrangement(arrid, node1.into_ddvalue());
            // let result = program.dump_arrangement(arrid);
            // println!("Print Arrangement {:?}", result);
            let result1 = program.dump_arrangement((3, 0));
            let result2 = program.dump_arrangement((3, 1));
            println!("Print Arrangement of (3, 0) is {:?}", result1);
            println!("Print Arrangement of (3, 1) is {:?}", result2);
        }
    }

    #[test]
    fn test_transitive_closure() {
        println!("Print out arguments: {:?}", std::env::args());
        // let nodes_num: usize = std::env::args().nth(1).unwrap_or("200".to_string()).parse().unwrap_or(100);
        // let edges_num: usize = std::env::args().nth(2).unwrap_or("300".to_string()).parse().unwrap_or(20);
        // let debug = std::env::args().nth(3).unwrap_or("debug".to_string()) == "debug";
        let nodes_num = 200;
        let edges_num = 300;
        let debug = true;

        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        let mut edge_set = HashSet::new();

        let mut i = 0;
        while i < edges_num {
            let num1 = rng1.gen_range(0, nodes_num);
            let num2 = rng1.gen_range(0, nodes_num);
            let edge = (num1 as u32, num2 as u32);
            // In case duplicates are generated
            if !edge_set.contains(&edge) {
                edge_set.insert(edge);
                i += 1;
            }
        }

        let edges: Vec<(u32, u32)> = edge_set.into_iter().collect();

        let mut terms = vec![];
        for (src, dst) in edges.iter() {
            let term_ast = term(&format!("Edge(Node({}), Node({}))~", src, dst)[..]).unwrap().1;
            let record = term_ast.into_record();
            let term = AtomicTerm::from_record(&record).unwrap();
            terms.push(term);
        }

        if debug {
            for term in terms.iter() { println!("Initial input: {:?}", term); }
        }

        let path = Path::new("./tests/testcase/p0.4ml");
        let content = fs::read_to_string(path).unwrap() + "EOF";
        let (_, program_ast) = parse_program(&content);
          
        let env: Env = program_ast.build_env();
        // let graph = env.get_domain_by_name("Graph").unwrap();

        let mut engine = FormulaExecEngine::new(env);
        let df = engine.dataflow_by_name("Graph");
        df.run();
        
        let timer = std::time::Instant::now();
        df.insert_terms(terms);
        println!("Graph has {} nodes and {} edges. Running Time: {} milliseconds", 
            nodes_num, edges_num, timer.elapsed().as_millis());
    }
}