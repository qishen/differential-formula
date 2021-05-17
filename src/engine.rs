use std::borrow::Cow;
use std::convert::*;
use std::sync::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{BTreeMap, HashSet, HashMap};

use im::OrdSet;
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
use crate::util::*;
use crate::util::map::*;


/**
1. Formula Composite types translate to `Relation` in ddlog
2. Formula terms translate to `DDValue` that can run in the dataflow. How about `Record`?
3. Translate conjunction of predicates. What's the best way to do optimal joins? 
    Pred1(x1, x2,...,xn, y1, y2,..., yn), Pred2(y1, y2,..., yn, z1, z2,...,zn)
    Create two arrangements 
        1. ((y1,..,yn), (x1,..,xn)) arranged by (y1,..,yn) into ((y1,..yn), Pred1)
        2. ((y1,..,yn), (z1,..,zn)) arranged by (y1,..,yn) into ((y1,..yn), Pred2)
        
**/

struct FormulaExecEngine {
    program: Program
}

impl FormulaExecEngine {
    fn new<T: TermStructure>(env: Env<T>) -> Self {
        for (name, domain) in env.domain_map {
            let meta = domain.meta_info();
            for rule in meta.rules() {

            }
        }

        // let rel = FormulaExecEngine::join_two_preds();

        let program: Program = Program {
            nodes: vec![
                // They have to be in the right order.
                // ProgNode::Rel { rel: rel0 },
                // ProgNode::Rel { rel: rel1 }, 
                // ProgNode::Rel { rel }
                ],
            delayed_rels: vec![],
            init_data: vec![],
        };

        FormulaExecEngine { program }
        
    }

    // fn into_relation(rawtype: RawType) -> Option<Relation> {
    //     if let RawType::CompositeType(ctype) = rawtype {
    //         // let relset0: Arc<Mutex<Delta<Edge>>> = Arc::new(Mutex::new(BTreeMap::default()));
    //         let rel = Relation {
    //             name: Cow::from(ctype.type_name()),
    //             input: true,
    //             distinct: false,
    //             caching_mode: CachingMode::Set,
    //             key_func: None,
    //             id: 0,
    //             rules: vec![],
    //             arrangements: Vec::new(),
    //             change_cb: None,
    //             // change_cb: Some(Arc::new(move |_, v, w| set_update("Edge", &relset0, v, w))),
    //         };
    //         Some(rel)
    //     } else { None }
    // }

    fn into_rules<T>(rule: &Rule<T>) -> DDRule where T: TermStructure {
        for pred in rule.predicate_constraints() {

        }
        todo!()
    }

    // Let's assume both are positive predicates.
    fn join_two_preds(t1: AtomicTerm, t2: AtomicTerm) -> (Relation, Relation) {
        /*
        Convert from 3D to 2D and generate a new relation.
        Z(A(a), B(b), U(m, x, y)) :- X(M(N(a), N(b)), D(m), Y(n, m)), Y(P(a), Q(P(b, m, y)), x).
        T1(a, b, m, n) :- X(M(N(a), N(b)), D(m), Y(n, m)).
        Flatten out: X(v1, v2, v3), v1 = M(v4, v5), v2 = D(m), v3 = Y(n, m), v4 = N(a), v5 = N(b).
        T2(a, b, m, x, y) :- Y(P(a), Q(J(b, m, y)), x)
        (key: (a, b, m), left: (n), right: (x, y))

        DDValue -> (DDValue, DDValue)
        X(M(N(a), N(b)), D(m), Y(n, m)) -> ((a, b, m), (n))
        Y(P(a), Q(P(b, m, y)), x) -> ((a, b, m), (x, y))
        */
        let vars1 = t1.variables();
        let vars2 = t2.variables();
        println!("t1 vars: {:?} and t2 vars: {:?}", vars1, vars2);

        let shared_vars: Vec<AtomicTerm> = vars1.clone().into_iter().filter(|x| {
            vars2.contains(x) 
        }).collect();
        let shared_vars_copy = shared_vars.clone();

        let left_vars: Vec<AtomicTerm> = vars1.clone().into_iter().filter(|x| 
            !vars2.contains(x)
        ).collect();

        let right_vars: Vec<AtomicTerm> = vars2.clone().into_iter().filter(|x| 
            !vars1.contains(x)
        ).collect();
        println!("shared vars: {:?}, left vars: {:?}, right vars: {:?}", shared_vars, left_vars, right_vars);

        // e.g. Edge(Node(a), b), Edge(c, b) 
        let arr1 = Arrangement::Map {
            name: Cow::from("(key, left)"),
            afun: Arc::new(move |v: DDValue| -> Option<(DDValue, DDValue)> {
                let term = <AtomicTerm>::from_ddvalue(v);
                if let Some(term_match) = t1.clone().match_to(&term) {
                    let shared_match: Vec<AtomicTerm> = shared_vars.clone().iter().map(|x| { 
                        term_match.get(&x).unwrap().clone().clone() 
                    }).collect();
                    let left_match: Vec<AtomicTerm> = left_vars.clone().iter().map(|x| {
                        term_match.get(&x).unwrap().clone().clone()
                    }).collect();
                    let key = shared_match.into_ddvalue(); 
                    let val = left_match.into_ddvalue();
                    println!("Left match {} to predicate {} and get {:?}", term, t1, (term_match, &key, &val));
                    return Some((key, val))
                }
                return None;
            }),
            queryable: false,
        };

        let arr2 = Arrangement::Map {
            name: Cow::from("(key, right)"),
            afun: Arc::new(move |v: DDValue| -> Option<(DDValue, DDValue)> {
                let term = <AtomicTerm>::from_ddvalue(v);
                if let Some(term_match) = t2.clone().match_to(&term) {
                    let shared_match: Vec<AtomicTerm> = shared_vars_copy.clone().iter().map(|x| { 
                        term_match.get(&x).unwrap().clone().clone() 
                    }).collect();
                    let right_match: Vec<AtomicTerm> = right_vars.clone().iter().map(|x| {
                        // println!("Term match is {:?} and right var is {}", term_match, x);
                        term_match.get(&x).unwrap().clone().clone()
                    }).collect();
                    let key = shared_match.into_ddvalue(); 
                    let val = right_match.into_ddvalue();
                    println!("Right match {} to predicate {} and get {:?}", term, t2, (term_match, &key, &val));
                    return Some((key, val))
                }
                return None;
            }),
            queryable: false,
        };

        // arr1 = (0, 0) joins arr2 = (0, 1) where the first number is the relation Id and the second
        // number is the index of arrangement inside current relation.
        let jrule = DDRule::ArrangementRule {
            description: Cow::from("Join arr1 and arr2 together!"),
            arr: (0, 0),
            xform: XFormArrangement::Join {
                description: Cow::from("whatever"),
                ffun: None,
                arrangement: (0, 1),
                jfun: {
                    // key, v1, v2 are DDValue converted from a vector of terms.
                    fn join_func(key: &DDValue, v1: &DDValue, v2: &DDValue) -> Option<DDValue> {
                        let key_terms = <Vec<AtomicTerm>>::from_ddvalue(key.clone());
                        let left_terms = <Vec<AtomicTerm>>::from_ddvalue(v1.clone());
                        let right_terms = <Vec<AtomicTerm>>::from_ddvalue(v2.clone());

                        println!("Join by key: {:?}, left: {:?}, right: {:?}", 
                            key_terms, left_terms, right_terms);

                        Some(key.clone())
                    }
                    join_func
                },
                next: Box::new(None)
            }
        };

        // Well, let's say there is only one relation to represent atomic terms.
        let dummy_rel = Relation {
            name: Cow::from("AtomicTerm"),
            input: true,
            distinct: false,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 0,
            rules: vec![],
            arrangements: vec![arr1, arr2],
            change_cb: None,
        };

        let rel = Relation {
            name: Cow::from("AtomicTerm"),
            input: false,
            distinct: false,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 1,
            rules: vec![jrule],
            arrangements: vec![],
            change_cb: None,
        };

        return (dummy_rel, rel);
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
          
        let terms = program_ast.model_ast_map.get("m").unwrap().clone().models;
        for term_ast in terms {
            let record: Record = term_ast.into_record();
            println!("Record: {}", record);
        }
          
        let env: Env<AtomicTerm> = program_ast.build_env();
        let graph = env.get_domain_by_name("Graph").unwrap();
        let m = env.get_model_by_name("m").unwrap();
        println!("{:#?}", graph);

        let rule1 = graph.meta_info().rules().get(1).unwrap().clone();
        let cons = rule1.predicate_constraints();
        let terms: Vec<AtomicTerm> = cons.iter().map(|con| {
            match con {
                Constraint::Predicate(pred) => {
                    pred.term.clone()
                },
                _ => Default::default()
            }
        }).collect();
        println!("Two terms in rule: {:?}", terms);

        let t1 = terms.get(0).unwrap().clone();
        let t2 = terms.get(1).unwrap().clone();

        let (dummy_rel, rel) = FormulaExecEngine::join_two_preds(t1, t2);

        let program: Program = Program {
            nodes: vec![
                // They have to be in the right order.
                // ProgNode::Rel { rel: rel0 },
                // ProgNode::Rel { rel: rel1 }, 
                ProgNode::Rel { rel: dummy_rel },
                ProgNode::Rel { rel: rel }
                ],
            delayed_rels: vec![],
            init_data: vec![],
        };

        let mut running = program.run(1).unwrap();
        running.transaction_start().unwrap();
        for term in m.terms() {
            println!("Going to insert the term: {}", term);
            running.insert(0, term.clone().into_ddvalue()).unwrap();
        }
        running.transaction_commit().unwrap();
    }
}