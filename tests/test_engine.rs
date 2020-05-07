extern crate differential_formula;

use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::module::*;
use differential_formula::type_system::*;

use std::fs;
use std::path::Path;
use std::collections::HashMap;

use rand::{Rng, SeedableRng, StdRng};


fn load_program(file_path: &str) -> DDEngine {
    let path = Path::new(file_path);
    let content = fs::read_to_string(path).unwrap();
    
    let mut engine = DDEngine::new();
    engine.inspect = true;
    engine.install(content);
    return engine;
}

#[test]
fn test_ddengine_1() {
    let rule = "Path(a, b) :- Edge(a, b).
        //Path(a, c) :- Path(a, b), Path(b, c).";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_2() {
    let rule = "Edge(a, c) :- Edge(a, b), Edge(b, c).";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_3() {
    let rule = "
        Edge(a, d) :- Edge(a, b), Edge(b, c), Edge(c, d).
    ";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_4() {
    let rule = "
        Path(a, b) :- Edge(a, b).
        Path(a, c) :- Path(a, b), Path(b, c).
        Nocycle(u) :- u is Node(_), no Path(u, u).
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();

    /*

    let (domain, mut session) = create_session(rules4, MODEL1);

    let edge00 = session.parse_term_str("Edge(Node(0), Node(0))").unwrap();
    let edge22 = session.parse_term_str("Edge(Node(2), Node(2))").unwrap();
    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();

    // Need to explicitly declare Node(5) to use it even though edge45 has it as argument.
    let node5 = session.parse_term_str("Node(5)").unwrap();
    
    session.add_terms(vec![node5, edge45]);
    session.add_term(edge22);
    session.remove_term(edge00);
    */
}

#[test]
fn test_ddengine_5() {
    let rule = "
        Line(a, b, c, d) :- Edge(a, b), Edge(c, d).
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}


#[test]
fn test_ddengine_6() {
    let rule = "
        TwoEdge(x, y) :- x is Edge(a, b), y is Edge(b, c).
    ";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_6x() {
    let rule = "
        TwoEdge(x, y) :- x is Edge(_, _), y is Edge(_, _).
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_7() {
    let rule = "
        TwoEdge(x, x, square) :- x is Edge(c, d), 
                                 aggr = count({Edge(a, a), b | Edge(a, b)}), 
                                 square = aggr * aggr, aggr * 2 = 20 .
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();

    /*
    let (domain, mut session) = create_session(rules7, MODEL1);

    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

    session.add_terms(vec![edge45, edge56]);*/
}

#[test]
fn test_ddengine_7x() {
    let rule = "
        Node(num) :- x is Node(_), num = aggr + 100, aggr = maxAll(1000, { k | Edge(_, Node(k)) }).
    ";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();

    /*
    let (domain, mut session) = create_session(rules7x, MODEL1);

    let edge34 = session.parse_term_str("Edge(Node(3), Node(4))").unwrap();
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

    session.add_term(edge56.clone());

    session.remove_terms(vec![edge34, edge56]);*/
}

#[test]
fn test_ddengine_8() {
    let rule = "
        Edge(x.src, d) :- x is Edge(a, b), y is Edge(b, c), Edge(y.dst, d).
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_ddengine_8x() {
    let rule = "
        Edge(a, c) :- x is Edge(a, y.src), y is Edge(x.dst, c).
    ";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
// TODO: Fix the constraint classification in rule.
fn test_ddengine_9() {
    // Let's try a nested aggregation.
    let rule = "
    TwoEdge(x, x, num) :- x is Edge(c, d), aggr1 = count({ n | n is Node(_), aggr2 = count({ x | x is Edge(a, b) }) }), num = aggr1 * 100 .
    ";
    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}


#[test]
fn test_ddengine_10() {
    let rule = "
    Node(num) :- num = aggr * 1000, aggr = count({Edge(a, a), b | Edge(a, b)}).
    ";

    let mut engine = load_program("./tests/testcase/p0.4ml");
    engine.add_rule("m", rule);
    let m = engine.env.get_model_by_name("m").unwrap().clone();
    let mut session = Session::new(m, &engine);
    session.load();
}

#[test]
fn test_print_modules() {
    let engine = load_program("./tests/testcase/p1.4ml");

    let dag = engine.env.get_domain_by_name("DAGs");
    println!("domain DAGs is {:?}", dag);

    let iso_dag = engine.env.get_domain_by_name("IsoDAGs");
    println!("domain IsoDAGs is {:#?}", iso_dag);

    let little_cycle = engine.env.get_model_by_name("LittleCycle").unwrap().clone();

    let pair = engine.env.get_model_by_name("Pair").unwrap().clone();
    println!("alias_map of model Pair is {:#?}", pair.alias_map);
    println!("model Pair is {:#?}", pair.terms());

    let del_transform = engine.env.get_transform_by_name("Del").unwrap().clone();
    for rule in del_transform.rules.iter() {
        println!("Original Rule: {}", rule);
    }
}


#[test]
fn test_transform_add() {
    let mut engine = load_program("./tests/testcase/p1.4ml");
    let transformation = engine.create_model_transformation("r = Add(100, LittleCycle)");
    let mut session = Session::new(transformation, &engine);
    session.load();

    //let v4 = session.create_term("GraphIn.V(4)").unwrap();
    //session.add_term(v4)
}

#[test]
fn test_transform_del() {
    let mut engine = load_program("./tests/testcase/p1.4ml");
    let transformation = engine.create_model_transformation("r = Del(1, LittleCycle)");
    let mut session = Session::new(transformation, &engine);
    session.load();

    let v4 = session.create_term("GraphIn.V(4)").unwrap();
    session.add_term(v4)
}

#[test]
fn test_transform_complete() {
    let mut engine = load_program("./tests/testcase/p1.4ml");
    let transformation = engine.create_model_transformation("r = Complete(LittleCycle)");
    let mut session = Session::new(transformation, &engine);
    session.load();
}

#[test]
fn test_transform_uglycopy() {
    let mut engine = load_program("./tests/testcase/p1.4ml");
    let transformation = engine.create_model_transformation("r = UglyCopy(LittleCycle)");
    let mut session = Session::new(transformation, &engine);
    session.load();
}

#[test]
fn test_transform_prettycopy() {
    let mut engine = load_program("./tests/testcase/p1.4ml");
    let transformation = engine.create_model_transformation("r = PrettyCopy(LittleCycle)");
    let mut session = Session::new(transformation, &engine);
    session.load();
}

#[test]
fn test_social_network() {
    let engine = load_program("./tests/samples/SocialNetwork.4ml");
    let model = engine.env.get_model_by_name("example").unwrap().clone();
    let mut session = Session::new(model, &engine);
    session.load();
}

#[test]
fn test_incremental_transitive_closure() {
    let mut engine = load_program("./tests/testcase/p0.4ml");
    let seed: &[_] = &[1, 2, 3, 0];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);

    // Example: cargo test test_incremental_transitive_closure -- --nocapture 100 100 1
    let nodes: usize = std::env::args().nth(3).unwrap_or("100".to_string()).parse().unwrap();
    let edges: usize = std::env::args().nth(4).unwrap_or("50".to_string()).parse().unwrap();
    let updated_edges: usize = std::env::args().nth(5).unwrap_or("20".to_string()).parse().unwrap();

    let m1 = engine.create_empty_model("m1", "Graph").clone();
    let mut session = Session::new(m1, &engine);
    let mut terms = vec![];

    for i in 0 .. edges {
        let num1 = rng1.gen_range(0, nodes);
        let num2 = rng1.gen_range(0, nodes);
        let edge_str = format!("Edge(Node({}), Node({}))", num1, num2);
        let edge_arc = session.create_term(&edge_str).unwrap();
        terms.push(edge_arc);
    }

    println!("Compute transitive closure with {} nodes and {} edges", nodes, edges);
    let timer = std::time::Instant::now();
    session.add_terms(terms);
    let d1 = timer.elapsed();

    let mut updated_terms = vec![];
    for _ in 0 .. updated_edges {
        let num1 = rng1.gen_range(0, nodes);
        let num2 = rng1.gen_range(0, nodes);
        let edge_str = format!("Edge(Node({}), Node({}))", num1, num2);
        let edge_arc = session.create_term(&edge_str).unwrap();
        updated_terms.push(edge_arc);
    }
    let timer = std::time::Instant::now();
    session.add_terms(updated_terms.clone());
    let d2 = timer.elapsed();

    let timer = std::time::Instant::now();
    session.remove_terms(updated_terms);
    let d3 = timer.elapsed();

    // Print out results for benchmark.
    println!("Initial computation finished in {:?}", d1);
    println!("Updates finished in {:?}", d2);
    println!("Remove all updates in {:?}", d3);
}