use std::{borrow::Cow, os::linux::raw};
use std::convert::*;
use std::sync::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{BTreeMap, HashSet, HashMap};

use bimap::*;
use timely::{dataflow::*, logging::ProgressEventTimestamp};
use differential_dataflow::*;
use differential_dataflow::input::InputSession;
use differential_dataflow::operators::join::*;
use differential_dataflow::operators::*;
use differential_dataflow::hashable::*;

// Import from local ddlog repo and APIs are subject to constant changes
use differential_datalog::ddval::*;
use differential_datalog::record::*;
use differential_datalog::program::{
    XFormArrangement,
    XFormCollection,
    Arrangement,
    CachingMode,
    ProgNode,
    Relation, 
    Program, 
    RunningProgram, 
    Rule as DDRule
};
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
    fn new(term: AtomicTerm, key: &Vec<AtomicTerm>, val: &Vec<AtomicTerm>) -> Option<Self> {
        let vars = term.variables();
        if !key.iter().all(|x| { vars.contains(x) }) { return None; }
        let (normalized_term, vmap) = term.normalize();

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

        Some(arr_key)
    }

    /// Check if a term can be matched to `FormulaArrKey` after normalization.
    fn is_matched(&self, term: &AtomicTerm) -> bool {
        let (normalized_term, _) = term.normalize();
        normalized_term == self.normalized_term
    }
}

struct FormulaTermIndex {
    relation_map: HashMap<String, Relation>,
    relation_id_map: BiMap<usize, String>,
    arr_map: HashMap<FormulaArrKey, Arrangement>,
    arr_id_map: BiMap<(usize, usize), FormulaArrKey>
}

impl FormulaTermIndex {
    fn new(relation_map: HashMap<String, Relation>) -> Self {
        let mut relation_id_map: BiMap<usize, String> = BiMap::new();
        for (name, relation) in relation_map.iter() {
            let rid = relation.id.clone();
            relation_id_map.insert(rid, name.clone());
        }

        FormulaTermIndex {
            relation_map,
            relation_id_map,
            arr_map: HashMap::new(),
            arr_id_map: BiMap::new()
        }
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
        let input_relation_name = format!("{}_input", pred.type_id());
        let relation_id = self.relation_id_map.get_by_right(&input_relation_name).unwrap().clone();
        let relation = self.relation_map.get_mut(&input_relation_name).unwrap();
        let arrkey = FormulaArrKey::new(pred.clone(), key, val).unwrap();
        let arr = Self::into_arrangment(pred, key, val);

        // Add new arrangement to the end of `arrangements` in this relation.
        let arr_id = (relation_id, relation.arrangements.len());
        relation.arrangements.push(arr.clone());

        self.arr_id_map.insert(arr_id, arrkey.clone());
        self.arr_map.insert(arrkey, arr);
    }

    fn add_rule(&mut self, rid: usize, rule: DDRule) -> bool {
        if let Some(relation_name) = self.relation_id_map.get_by_left(&rid) {
            let relation = self.relation_map.get_mut(relation_name).unwrap();
            relation.rules.push(rule);
            true
        } else { false } 
    }

    fn create_join_rule(&mut self, t1: &AtomicTerm, t2: &AtomicTerm) -> DDRule {
        // arr1 = (0, 0) joins arr2 = (0, 1) where the first number is the relation Id and the second
        // number is the index of arrangement inside current relation.
        let (key, left, right) = t1.variable_diff(t2);
        let arr_key1 = FormulaArrKey::new(t1.clone(), &key, &left).unwrap();
        let arr_key2 = FormulaArrKey::new(t2.clone(), &key, &right).unwrap();
        // println!("Find {:?} in {:?}", (&arr_key1, &arr_key2), self.arr_id_map);
        let arr_id1 = self.arrid_by_key(&arr_key1).unwrap();
        let arr_id2 = self.arrid_by_key(&arr_key2).unwrap();

        let join_rule = DDRule::ArrangementRule {
            description: Cow::from(format!("Join {} and {}", t1, t2)),
            arr: arr_id1,
            xform: XFormArrangement::Join {
                description: Cow::from("whatever"),
                ffun: None,
                arrangement: arr_id2,
                jfun: |key: &DDValue, v1: &DDValue, v2: &DDValue| -> Option<DDValue> {
                    // We cannot move extern values into the closure here.
                    let key_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(key);
                    let left_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(v1);
                    let right_terms = <Vec<AtomicTerm>>::from_ddvalue_ref(v2);
                    let join = (key_terms.clone(), left_terms.clone(), right_terms.clone()).into_ddvalue();
                    println!("join result: {:?}", join);
                    Some(join)
                },
                next: Box::new(None)
            }
        };

        join_rule
    }

    /// `key_vars` and `val_vars` must be disjoint subset of the variables in `matching_term` 
    /// e.g. Edge(Node(a), b), Edge(c, b) where key is variable `b` and the val is `a` or `c`. 
    fn into_arrangment(
        matching_term: &AtomicTerm, 
        key: &Vec<AtomicTerm>, 
        val: &Vec<AtomicTerm>
    ) -> Arrangement {
        let key_vars = key.clone();
        let val_vars = val.clone();
        let matching_term = matching_term.clone();
        let arr = Arrangement::Map {
            name: Cow::from(format!("Matching {} with key: {:?} and val: {:?}", 
                matching_term, 
                key_vars, 
                val_vars)
            ),
            afun: Arc::new(move |v: DDValue| -> Option<(DDValue, DDValue)> {
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

                    println!("Left match {} to predicate {} and get {:?}", term, 
                        matching_term, (term_match, &key, &val));

                    return Some((key, val))
                }
                return None;
            }),
            queryable: false,
        };

        arr
    }
}

struct DomainDataflow {
    index: FormulaTermIndex,
    running_program: Option<RunningProgram>,
}

impl DomainDataflow {
    fn is_running(&self) -> bool {
        self.running_program.is_some()
    }
    
    fn run(&mut self) {
        // The relations must be added in the right order otherwise may fail to load
        // arrangement from dependent relation.
        let nodes: Vec<ProgNode> = self.index.relation_map.iter().map(|(_, relation)| {
            ProgNode::Rel { rel: relation.clone() }
        }).collect();
        let program: Program = Program {
            nodes,
            delayed_rels: vec![],
            init_data: vec![],
        };
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
            // Must add the new terms into the input relation.
            let type_name = format!("{}_input", term.type_id());
            let rid = self.index.relation_id_map.get_by_right(&type_name).unwrap();
            println!("Going to insert the term: {}", term);
            running_program.insert(*rid, term.into_ddvalue()).unwrap();
        }
        running_program.transaction_commit().unwrap();
    }

    // fn add_rule(&mut self) {
    //     let filter_rule = DDRule::CollectionRule {
    //         description: Cow::from("hi"),
    //         rel: 0,
    //         xform: Some(XFormCollection::Filter {
    //             description: Cow::from("hi"),
    //             ffun: {
    //                 fn filter_func(v: &DDValue) -> bool {
    //                     let term = <AtomicTerm>::from_ddvalue_ref(v);
    //                     term.type_id().as_ref() == "Edge" 
    //                 }
    //                 filter_func
    //             },
    //             next: Box::new(None)
    //         })
    //     };
    // }
}

struct FormulaExecEngine<T: TermStructure> {
    env: Env<T>,
    domains: HashMap<String, DomainDataflow>
}

impl FormulaExecEngine<AtomicTerm> {
    fn dataflow_by_name(&mut self, name: &str) -> &mut DomainDataflow {
        self.domains.get_mut(name).unwrap()
    }

    fn new(env: Env<AtomicTerm>) -> Self {
        let mut dataflows = HashMap::new();
        for (domain_name, domain) in env.domain_map.iter() {
            let meta = domain.meta_info();
            let mut relation_counter = 0;
            // `Relation`, Relation id and a unique string to represent relation.
            let mut relation_map = HashMap::new();
            // All relations in stratum 0 only for inputs and we may want to build some
            // arrangements for each relation.
            // type `Path` is represented by two relations that one only for input and 
            // the other for the following reasoning and derivation from input or previous stratums.
            for (name, _) in meta.composite_types().into_iter() {
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
                    distinct: true,
                    caching_mode: CachingMode::Set,
                    key_func: None,
                    id: relation_id,
                    rules: vec![],
                    arrangements: vec![],
                    change_cb: None,
                };

                println!("input relation name: {}_input id: {}", name, input_relation_id);
                println!("relation name: {} id: {}", name, relation_id);

                relation_map.insert(format!("{}_input", name), input_rel);
                relation_map.insert(name.clone(), rel);
            }

            let mut index = FormulaTermIndex::new(relation_map);

            // TODO: Assume all rules are sorted and in the same Stratum 1 for now.
            for rule in meta.rules() {
                let pred_terms: Vec<AtomicTerm> = rule.predicate_constraints().iter().filter_map(|constraint| {
                    match *constraint {
                        Constraint::Predicate(pred) => {
                            if !pred.negated { Some(pred.term.clone()) } else { None }
                        },
                        _ => None
                    }
                }).collect();
                
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

                let join_rule = index.create_join_rule(&t1, &t2);
                let rid = index.rid_by_name(t1.type_id().as_ref()).unwrap();
                index.add_rule(rid, join_rule);
            }
            
            // TODO: How about stratums after Stratum 1.

            let dataflow = DomainDataflow {
                index,
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

    #[test]
    fn test_parse_models() {
        let path = Path::new("./tests/testcase/p0.4ml");
        let content = fs::read_to_string(path).unwrap() + "EOF";
        let (_, program_ast) = parse_program(&content);
          
        // let terms = program_ast.model_ast_map.get("m").unwrap().clone().models;
        // for term_ast in terms {
        //     let record: Record = term_ast.into_record();
        //     println!("Record: {}", record);
        // }
          
        let env: Env<AtomicTerm> = program_ast.build_env();
        let graph = env.get_domain_by_name("Graph").unwrap();
        let m = env.get_model_by_name("m").unwrap();
        println!("{:#?}", graph);

        let terms: Vec<AtomicTerm> = m.terms().into_iter().map(|x| x.clone()).collect();

        let mut engine = FormulaExecEngine::new(env);
        let df = engine.dataflow_by_name("Graph");
        df.run();
        df.insert_terms(terms);

        // let rule1 = graph.meta_info().rules().get(1).unwrap().clone();
        // let cons = rule1.predicate_constraints();
        // let terms: Vec<AtomicTerm> = cons.iter().map(|con| {
        //     match con {
        //         Constraint::Predicate(pred) => {
        //             pred.term.clone()
        //         },
        //         _ => Default::default()
        //     }
        // }).collect();
        // println!("Two terms in rule: {:?}", terms);

        // let t1 = terms.get(0).unwrap().clone();
        // let t2 = terms.get(1).unwrap().clone();

        // let (dummy_rel, rel) = FormulaExecEngine::join_two_preds(t1, t2);

        // let program: Program = Program {
        //     nodes: vec![
        //         // They have to be in the right order.
        //         // ProgNode::Rel { rel: rel0 },
        //         // ProgNode::Rel { rel: rel1 }, 
        //         ProgNode::Rel { rel: dummy_rel },
        //         ProgNode::Rel { rel: rel }
        //         ],
        //     delayed_rels: vec![],
        //     init_data: vec![],
        // };

        // let mut running = program.run(1).unwrap();
        // running.transaction_start().unwrap();
        // for term in m.terms() {
        //     println!("Going to insert the term: {}", term);
        //     running.insert(0, term.clone().into_ddvalue()).unwrap();
        // }
        // running.transaction_commit().unwrap();
    }
}