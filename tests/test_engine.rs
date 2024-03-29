// #![type_length_limit="1120927"]
use std::borrow::Cow;
use std::collections::btree_map::{BTreeMap, Entry};
// use std::collections::btree_set::BTreeSet;
// use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use std::fs;
// use std::path::Path;

use rand::{Rng, SeedableRng, StdRng};
use fnv::FnvHashMap;


use timely::communication::Allocator;
use timely::dataflow::scopes::*;
use timely::worker::Worker;


use differential_dataflow::operators::Join;
use differential_dataflow::Collection;


use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::module::{Program as FProgram, Env};
use differential_formula::type_system::*;

use differential_datalog::ddval::*;
use differential_datalog::program::*;
use ::differential_datalog::record::FromRecord;
use ::differential_datalog::record::IntoRecord;
use ::differential_datalog::record::Mutator;
use differential_datalog_test::test_value::*;
use ddlog_derive::*;

use serde::Deserialize;
use serde::Serialize;

#[derive( Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, 
          IntoRecord, FromRecord, Mutator, Serialize, Deserialize)]
pub struct E {
    pub src: N, 
    pub dst: N,
}

#[derive( Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, 
          IntoRecord, FromRecord, Mutator, Serialize, Deserialize)]
pub struct N {
    pub id: u32,
}

#[derive( Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, 
          IntoRecord, FromRecord, Mutator, Serialize, Deserialize)]
pub struct P {
    pub src: N,
    pub dst: N,
}

#[derive( Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, 
          IntoRecord, FromRecord, Mutator, Serialize, Deserialize)]
#[ddlog(rename = "Edge")]
pub struct Edge {
    pub src: u64,
    pub dst: u64
}
impl abomonation::Abomonation for Edge{}
impl ::std::fmt::Display for Edge {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Edge{src,dst} => {
                __formatter.write_str("Edge{")?;
                ::std::fmt::Debug::fmt(src, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(dst, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, IntoRecord, Mutator, Default, Serialize, Deserialize, FromRecord)]
#[ddlog(rename = "NoCycle")]
pub struct NoCycle {
    pub node: u32
}
impl abomonation::Abomonation for NoCycle{}
impl ::std::fmt::Display for NoCycle {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            NoCycle{node} => {
                __formatter.write_str("NoCycle{")?;
                ::std::fmt::Debug::fmt(node, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for NoCycle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, IntoRecord, Mutator, Default, Serialize, Deserialize, FromRecord)]
#[ddlog(rename = "Path")]
pub struct Path {
    pub src: u64,
    pub dst: u64
}
impl abomonation::Abomonation for Path{}
impl ::std::fmt::Display for Path {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Path{src,dst} => {
                __formatter.write_str("Path{")?;
                ::std::fmt::Debug::fmt(src, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(dst, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for Path {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}

// fn load_program(file_path: &str) -> DDEngine<AtomicTerm> {
//     let path = Path::new(file_path);
//     let content = fs::read_to_string(path).unwrap();
//     let env = AtomicTerm::load_program(content);
//     let mut engine = DDEngine::new(env);
//     engine.inspect = true;
//     return engine;
// }

// fn load_program(file_path: &str) -> Env<AtomicTerm> {
//     let path = Path::new(file_path);
//     let content = fs::read_to_string(path).unwrap();
//     let env = AtomicTerm::load_program(content);
//     env
// }

const TEST_SIZE: u64 = 1000;

type Delta<T> = BTreeMap<T, Weight>;

fn set_update<T>(_rel: &str, s: &Arc<Mutex<Delta<T>>>, x: &DDValue, w: Weight)
where
    T: Ord + DDValConvert + Clone + 'static,
{
    let mut delta = s.lock().unwrap();

    let entry = delta.entry(T::from_ddvalue_ref(x).clone());
    match entry {
        Entry::Vacant(vacant) => {
            vacant.insert(w);
        }
        Entry::Occupied(mut occupied) => {
            if *occupied.get() == -w {
                occupied.remove();
            } else {
                *occupied.get_mut() += w;
            }
        }
    };

    //println!("set_update({}) {:?} {}", rel, *x, insert);
}

// #[test]
// fn test_ddengine_1() {
//     let rule = "Path(a, b) :- Edge(a, b).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

#[test]
fn test_ddlog_engine() {
    let nthreads = 1;
    fn ifun(_v: &DDValue, _ts: TupleTS, _w: Weight) {}

    let relset1: Arc<Mutex<Delta<U64>>> = Arc::new(Mutex::new(BTreeMap::default()));
    let rel1 = {
        let relset1 = relset1.clone();
        Relation {
            name: Cow::from("T1"),
            input: true,
            distinct: true,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 1,
            rules: Vec::new(),
            arrangements: Vec::new(),
            change_cb: Some(Arc::new(move |_, v, w| set_update("T1", &relset1, v, w))),
        }
    };

    let relset2: Arc<Mutex<Delta<U64>>> = Arc::new(Mutex::new(BTreeMap::default()));
    let rel2 = {
        let relset2 = relset2.clone();
        Relation {
            name: Cow::from("T2"),
            input: false,
            distinct: true,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 2,
            rules: vec![Rule::CollectionRule {
                description: Cow::Borrowed("T2.R1"),
                rel: 1,
                xform: Some(XFormCollection::Inspect {
                    description: Cow::from("Inspect"),
                    ifun: ifun as InspectFunc,
                    next: Box::new(None),
                }),
            }],
            arrangements: Vec::new(),
            change_cb: Some(Arc::new(move |_, v, w| set_update("T2", &relset2, v, w))),
        }
    };

    let prog: Program = Program {
        nodes: vec![ProgNode::Rel { rel: rel1 }, ProgNode::Rel { rel: rel2 }],
        delayed_rels: vec![],
        init_data: vec![],
    };

    let mut running = prog.run(nthreads).unwrap();

    /* 1. Populate T1 */
    let vals: Vec<u64> = (0..TEST_SIZE).collect();
    let set: BTreeMap<_, _> = vals.iter().map(|x| (U64(*x), 1)).collect();

    running.transaction_start().unwrap();
    for x in set.keys() {
        running.insert(1, x.clone().into_ddvalue()).unwrap();
        //assert_eq!(running.relation_clone_content(1).unwrap(),
        //           running.relation_clone_content(2).unwrap());
    }
    running.transaction_commit().unwrap();

    assert_eq!(*relset1.lock().unwrap(), set);
    assert_eq!(*relset1.lock().unwrap(), *relset2.lock().unwrap());

    /* 2. Clear T1 */
    running.transaction_start().unwrap();
    for x in set.keys() {
        running.delete_value(1, x.clone().into_ddvalue()).unwrap();
        //        assert_eq!(running.relation_clone_content(1).unwrap(),
        //                   running.relation_clone_content(2).unwrap());
    }
    running.transaction_commit().unwrap();

    assert_eq!(relset2.lock().unwrap().len(), 0);
    assert_eq!(*relset1.lock().unwrap(), *relset2.lock().unwrap());

    /* 3. Rollback */
    running.transaction_start().unwrap();
    for x in set.keys() {
        running.insert(1, x.clone().into_ddvalue()).unwrap();
        //assert_eq!(running.relation_clone_content(1).unwrap(),
        //           running.relation_clone_content(2).unwrap());
    }
    running.transaction_rollback().unwrap();

    assert_eq!(relset1.lock().unwrap().len(), 0);
    assert_eq!(*relset1.lock().unwrap(), *relset2.lock().unwrap());

    running.stop().unwrap();
}



#[test]
fn test_simple_rule() {
    let nthreads = 1;

    let relset0: Arc<Mutex<Delta<Edge>>> = Arc::new(Mutex::new(BTreeMap::default()));
    let rel0 = {
        let relset0 = relset0.clone();
        Relation {
            name: Cow::from("Edge"),
            input: true,
            distinct: false,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 0,
            rules: vec![],
            arrangements: Vec::new(),
            change_cb: Some(Arc::new(move |_, v, w| set_update("Edge", &relset0, v, w))),
        }
    };

    let rule1 = Rule::CollectionRule {
        description: std::borrow::Cow::from("Path(.src=a, .dst=b) :- Edge(.src=a, .dst=b)."),
        rel: 0,
        xform: Some(XFormCollection::FilterMap{
            description: std::borrow::Cow::from("head of Path(.src=a, .dst=b) :- Edge(.src=a, .dst=b)."),
            fmfun: {fn __f(__v: DDValue) -> ::std::option::Option<DDValue>
            {
                // println!("debug rule1: {}", __v);
                let (ref a, ref b) = match *<Edge>::from_ddvalue_ref(&__v) {
                    Edge{src: ref a, dst: ref b} => ((*a).clone(), (*b).clone()),
                    _ => return None
                };
                let path = Path {src: (*a).clone(), dst: (*b).clone()};
                Some(path.into_ddvalue())
                // Some(1.into_ddvalue())
            }
            __f},
            next: Box::new(None)
        })
    };
    
    // Relation `Path`
    let relset1: Arc<Mutex<Delta<Path>>> = Arc::new(Mutex::new(BTreeMap::default()));
    let rel1 = {
        let relset1 = relset1.clone();
        Relation {
            name: Cow::from("Path"),
            input: true,
            distinct: false,
            caching_mode: CachingMode::Set,
            key_func: None,
            id: 1,
            rules: vec![ rule1 ],
            arrangements: Vec::new(),
            // change_cb: None,
            change_cb: Some(Arc::new(move |_, v, w| set_update("Path", &relset1, v, w))),
        }
    };

    let prog: Program = Program {
        nodes: vec![
            // They have to be in the right order.
            ProgNode::Rel { rel: rel0 },
            ProgNode::Rel { rel: rel1 }, 
            ],
        delayed_rels: vec![],
        init_data: vec![],
    };

    let mut running = prog.run(nthreads).unwrap();

    /* 1. Populate T1 */
    let vals: Vec<u64> = (0..10).collect();
    let set: BTreeMap<_, _> = vals.iter().map(|x| {
        // They will be converted into DDValue later.
        (Edge {src: *x, dst: *x + 1}, 1)
    }).collect();

    running.transaction_start().unwrap();
    for x in set.keys() {
        let ddvalue = x.clone().into_ddvalue();
        // let ddvalue = 100.into_ddvalue();

        println!("{}", ddvalue);

        running.insert(0, ddvalue).unwrap();
        //assert_eq!(running.relation_clone_content(1).unwrap(),
        //           running.relation_clone_content(2).unwrap());
    }

    running.transaction_commit().unwrap();

    for (i, x) in relset0.lock().unwrap().iter() {
        println!("Edge, key: {}, value: {}", i, x);
    }
    
    for (i, x) in relset1.lock().unwrap().iter() {
        println!("Path, key: {}, value: {}", i, x);
    }
}

// #[test]
// fn test_ddengine_2() {
//     let rule = "Edge(a, c) :- Edge(a, b), Edge(b, c).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_3() {
//     let rule = "Edge(a, d) :- Edge(a, b), Edge(b, c), Edge(c, d).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }


// #[test]
// fn test_ddengine_4() {
//     // The evaluation of the body only return a collection of binding with only
//     // aggregation result in it while `u` is ignored but head terms still require it.
//     // One solution is to auto-generate additional constraint u is Node(_) to hold it.
//     let rule = "Nocycle(u) :- u is Node(_), no Edge(u, u).";

//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();

//     let edge00 = session.create_term("Edge(Node(0), Node(0))").unwrap();
//     //let node100 = Arc::new(session.create_term("Node(100)").unwrap());
//     session.add_term(edge00);
//     //session.add_terms(vec![node100, edge00]);
// }

// #[test]
// fn test_ddengine_4x() {
//     let rule1 = "Path(a, b) :- Edge(a, b).";
//     let rule2 = "Path(a, c) :- Path(a, b), Path(b, c).";
//     let rule3 = "Nocycle(u) :- u is Node(_), no Path(u, u).";

//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule1);
//     engine.add_rule("m", rule2);
//     engine.add_rule("m", rule3);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();

//     let edge00 = session.create_term("Edge(Node(0), Node(0))").unwrap();
//     let edge22 = session.create_term("Edge(Node(2), Node(2))").unwrap();
//     let edge45 = session.create_term("Edge(Node(4), Node(5))").unwrap();

//     // Explicitly declare Node(5) to use even though edge45 has it as argument.
//     let node5 = session.create_term("Node(5)").unwrap();
    
//     //session.add_terms(vec![node5, edge45]);
//     //session.add_terms(vec![edge00.clone(), edge22]);
//     session.add_term(edge00.clone());

//     //session.remove_term(edge00.clone());
// }

// #[test]
// fn test_ddengine_5() {
//     let rule = "Line(a, b, c, d) :- Edge(a, b), Edge(c, d).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }


// #[test]
// fn test_ddengine_6() {
//     let rule = "TwoEdge(x, y) :- x is Edge(a, b), y is Edge(b, c).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_6x() {
//     let rule = "TwoEdge(x, y) :- x is Edge(_, _), y is Edge(_, _).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_7() {
//     let rule = "TwoEdge(x, x, square) :- x is Edge(c, d), 
//                                  aggr = count({Edge(a, a), b | Edge(a, b)}), 
//                                  square = aggr * aggr, aggr * 2 = 20 .
//     ";

//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();

//     /*
//     let (domain, mut session) = create_session(rules7, MODEL1);

//     let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();
//     let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

//     session.add_terms(vec![edge45, edge56]);*/
// }

// #[test]
// fn test_ddengine_7x() {
//     let rule = "Node(num) :- x is Node(_), num = aggr + 100, aggr = maxAll(1000, { k | Edge(_, Node(k)) }).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();

//     /*
//     let (domain, mut session) = create_session(rules7x, MODEL1);

//     let edge34 = session.parse_term_str("Edge(Node(3), Node(4))").unwrap();
//     let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

//     session.add_term(edge56.clone());

//     session.remove_terms(vec![edge34, edge56]);*/
// }

// #[test]
// fn test_ddengine_8() {
//     let rule = "Edge(x.src, d) :- x is Edge(a, b), y is Edge(b, c), Edge(y.dst, d).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_8x() {
//     let rule = "Edge(a, c) :- x is Edge(a, y.src), y is Edge(x.dst, c).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_9() {
//     // Let's try a nested aggregation.
//     // There are 5 nodes and 4 edges, so aggr1 should be 9 while the actual number of bindings derived from
//     // constraints inside set comprehension is 40 rather than 9 if not consolidated.
//     let rule = "TwoEdge(x, x, num) :- x is Edge(c, d), 
//     aggr1 = count({ n, e | aggr2 = maxAll(1000, { b | x is Node(b) }), e is Edge(_, _), n is Node(_), aggr2 = 4 }), 
//     num = aggr1 * 100 .";

//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_9x() {
//     // Let's try a nested aggregation that has shared variable between inner and outer scope.
//     // `e is Edge(d, _)` in which `d` is used in the outer scope as `x is Edge(c, d)`.
//     let rule = "TwoEdge(x, x, num) :- x is Edge(c, d), 
//     aggr1 = count({ n, e | aggr2 = maxAll(1000, { b | x is Node(b) }), e is Edge(d, _), n is Node(_), aggr2 = 4 }), 
//     num = aggr1 * 100 .";

//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }


// #[test]
// fn test_ddengine_10() {
//     let rule = "Node(num) :- num = aggr * 1000, Node(x), aggr = count({Edge(a, a), b | Edge(a, b)}).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_10x() {
//     // When outer scope has no constraints then create binding that only has aggregation result.
//     let rule = "Node(num) :- num = aggr * 1000, aggr = count({Edge(a, a), b | Edge(a, b)}).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_ddengine_11() {
//     // When outer scope has no constraints then create binding that only has aggregation result.
//     let rule = "same :- count({ b | Edge(a, b) }) * 2 = count({ Edge(a, a), b | Edge(a, b) }).";
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     engine.add_rule("m", rule);
//     let m = engine.env.get_model_by_name("m").unwrap().clone();
//     let mut session = Session::new(m, &engine);
//     session.load();
// }

// #[test]
// fn test_print_modules() {
//     let engine = load_program("./tests/testcase/p1.4ml");

//     let dag = engine.env.get_domain_by_name("DAGs");
//     println!("domain DAGs is {:?}", dag);

//     let iso_dag = engine.env.get_domain_by_name("IsoDAGs");
//     println!("domain IsoDAGs is {:#?}", iso_dag);

//     let little_cycle = engine.env.get_model_by_name("LittleCycle").unwrap().clone();

//     let pair = engine.env.get_model_by_name("Pair").unwrap().clone();
//     println!("alias_map of model Pair is {:#?}", pair.model_store().alias_map());
//     println!("model Pair is {:#?}", pair.terms());

//     let del_transform = engine.env.get_transform_by_name("Del").unwrap().clone();
//     for rule in del_transform.meta_info().rules().iter() {
//         println!("Original Rule: {}", rule);
//     }
// }


// #[test]
// fn test_transform_add() {
//     let mut engine = load_program("./tests/testcase/p1.4ml");
//     let transformation = engine.create_model_transformation("r = Add(100, LittleCycle)");
//     let mut session = Session::new(transformation, &engine);
//     session.load();

//     //let v4 = session.create_term("GraphIn.V(4)").unwrap();
//     //session.add_term(v4)
// }

// #[test]
// fn test_transform_del() {
//     let mut engine = load_program("./tests/testcase/p1.4ml");
//     let transformation = engine.create_model_transformation("r = Del(1, LittleCycle)");
//     let mut session = Session::new(transformation, &engine);
//     session.load();

//     let v4 = session.create_term("GraphIn.V(4)").unwrap();
//     session.add_term(v4);
// }

// #[test]
// fn test_transform_complete() {
//     let mut engine = load_program("./tests/testcase/p1.4ml");
//     let transformation = engine.create_model_transformation("r = Complete(LittleCycle)");
//     let mut session = Session::new(transformation, &engine);
//     session.load();
// }

// #[test]
// fn test_transform_uglycopy() {
//     let mut engine = load_program("./tests/testcase/p1.4ml");
//     let transformation = engine.create_model_transformation("r = UglyCopy(LittleCycle)");
//     let mut session = Session::new(transformation, &engine);
//     session.load();
// }

// #[test]
// fn test_transform_prettycopy() {
//     let mut engine = load_program("./tests/testcase/p1.4ml");
//     let transformation = engine.create_model_transformation("r = PrettyCopy(LittleCycle)");
//     let mut session = Session::new(transformation, &engine);
//     session.load();
// }

// #[test]
// fn test_social_network() {
//     let engine = load_program("./tests/samples/SocialNetwork.4ml");
//     let model = engine.env.get_model_by_name("example").unwrap().clone();
//     let mut session = Session::new(model, &engine);
//     session.load();
// }

// #[test]
// fn test_incremental_transitive_closure() {
//     let mut engine = load_program("./tests/testcase/p0.4ml");
//     let seed: &[_] = &[1, 2, 3, 0];
//     let mut rng1: StdRng = SeedableRng::from_seed(seed);

//     // Example: cargo test test_incremental_transitive_closure -- --nocapture 100 100 1
//     let nodes: usize = std::env::args().nth(3).unwrap_or("100".to_string()).parse().unwrap();
//     let edges: usize = std::env::args().nth(4).unwrap_or("50".to_string()).parse().unwrap();
//     let updated_edges: usize = std::env::args().nth(5).unwrap_or("20".to_string()).parse().unwrap();

//     let m1 = engine.create_empty_model("m1", "Graph");
//     engine.install_model(m1);
//     // Add a rule separately into model `m1` to compute transitive closure.
//     let rule0 = "Path(a, b) :- Edge(a, b).";
//     let rule1 = "Path(a, c) :- Path(a, b), Path(b, c).";
//     engine.add_rule("m1", rule0);
//     engine.add_rule("m1", rule1);
//     let m1 = engine.env.get_model_by_name("m1").unwrap().clone();
//     let mut session = Session::new(m1, &engine);
//     let mut terms = vec![];

//     for _ in 0 .. edges {
//         let num1 = rng1.gen_range(0, nodes);
//         let num2 = rng1.gen_range(0, nodes);
//         let edge_str = format!("Edge(Node({}), Node({}))", num1, num2);
//         let edge = session.create_term(&edge_str).unwrap();
//         terms.push(edge);
//     }

//     println!("Compute transitive closure with {} nodes and {} edges", nodes, edges);
//     let timer = std::time::Instant::now();
//     session.add_terms(terms);
//     let d1 = timer.elapsed();

//     let mut updated_terms = vec![];
//     for _ in 0 .. updated_edges {
//         let num1 = rng1.gen_range(0, nodes);
//         let num2 = rng1.gen_range(0, nodes);
//         let edge_str = format!("Edge(Node({}), Node({}))", num1, num2);
//         let edge = session.create_term(&edge_str).unwrap();
//         updated_terms.push(edge);
//     }
//     let timer = std::time::Instant::now();
//     session.add_terms(updated_terms.clone());
//     let d2 = timer.elapsed();

//     let timer = std::time::Instant::now();
//     session.remove_terms(updated_terms);
//     let d3 = timer.elapsed();

//     // Print out results for benchmark.
//     println!("Initial computation finished in {:?}", d1);
//     println!("Updates finished in {:?}", d2);
//     println!("Remove all updates in {:?}", d3);
// }