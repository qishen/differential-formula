extern crate differential_formula;

use differential_formula::engine::*;
use differential_formula::type_system::*;

use std::fs;
use std::path::Path;


static MODEL1: &str = "
model m of Graph {
    n0 is Node(0).
    n1 is Node(1).
    n2 is Node(2).
    n3 is Node(3).
    n4 is Node(4).

    Edge(n0, n0).
    Edge(n0, n1).
    Edge(n1, n2).
    //Edge(n2, n2).
    Edge(n2, n3).
    Edge(n3, n4).
}";


fn generate_graph_program(rules: &str, model: &str) -> String {
    let program = format!("
    domain Graph {{
        Node ::= new(name: Integer).
        Edge ::= new(src: Node, dst: Node).
        Path ::= new(src: Node, dst: Node).
        Line ::= new(a: Node, b: Node, c: Node, d: Node).
        Nocycle ::= new(node: Node).
        TwoEdge ::= new(first: Edge, second: Edge).

        {}
    }}

    {}
    ", rules, model);
    
    program
}

fn create_session(rules: &str, model: &str) -> (Domain, Session) {
    let program = generate_graph_program(rules, model);
    println!("{}", program);

    let mut engine = DDEngine::new();
    engine.inspect = true;

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(program);
    //  println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
    let model = engine.get_model("m".to_string()).unwrap().clone();

    let mut session = engine.create_session("Graph", Some("m"));
    session.load_model(model);
    (domain.clone(), session)
}

#[test]
fn test_ddengine_1() {
    let rules1 = "
        Path(a, b) :- Edge(a, b).
        //Path(a, c) :- Path(a, b), Path(b, c).
    ";

    let (domain, mut session) = create_session(rules1, MODEL1);
}

#[test]
fn test_ddengine_2() {
    let rules2 = "
        Edge(a, c) :- Edge(a, b), Edge(b, c).
    ";
    
    let (domain, mut session) = create_session(rules2, MODEL1);
    
    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

    session.add_terms(vec![edge45, edge56]);
}

#[test]
fn test_ddengine_3() {
    let rules3 = "
        Edge(a, d) :- Edge(a, b), Edge(b, c), Edge(c, d).
    ";

    let (domain, mut session) = create_session(rules3, MODEL1);
}

#[test]
fn test_ddengine_4() {
    let rules4 = "
        Path(a, b) :- Edge(a, b).
        Path(a, c) :- Path(a, b), Path(b, c).
        Nocycle(u) :- u is Node(_), no Path(u, u).
    ";

    let (domain, mut session) = create_session(rules4, MODEL1);

    let edge00 = session.parse_term_str("Edge(Node(0), Node(0))").unwrap();
    let edge22 = session.parse_term_str("Edge(Node(2), Node(2))").unwrap();
    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();

    // Need to explicitly declare Node(5) to use it even though edge45 has it as argument.
    let node5 = session.parse_term_str("Node(5)").unwrap();
    
    session.add_terms(vec![node5, edge45]);
    session.add_term(edge22);
    session.remove_term(edge00);
}

#[test]
fn test_ddengine_5() {
    let rules5 = "
        Line(a, b, c, d) :- Edge(a, b), Edge(c, d).
    ";

    let (domain, mut session) = create_session(rules5, MODEL1);
}


#[test]
fn test_ddengine_6() {
    let rules6 = "
        TwoEdge(x, y) :- x is Edge(a, b), y is Edge(b, c).
    ";

    let (domain, mut session) = create_session(rules6, MODEL1);
}

#[test]
fn test_ddengine_6x() {
    let rules6x = "
        TwoEdge(x, y) :- x is Edge(_, _), y is Edge(_, _).
    ";

    let (domain, mut session) = create_session(rules6x, MODEL1);
}

#[test]
fn test_ddengine_7() {
    let rules7 = "
        TwoEdge(x, x, square) :- x is Edge(c, d), 
                                 aggr = count({Edge(a, a), b | Edge(a, b)}), 
                                 square = aggr * aggr, aggr * 2 = 20 .
    ";

    let (domain, mut session) = create_session(rules7, MODEL1);

    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))").unwrap();
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

    session.add_terms(vec![edge45, edge56]);
}

#[test]
fn test_ddengine_7x() {
    let rules7x = "
        Node(num) :- x is Node(_), num = aggr + 100, aggr = maxAll(1000, { k | Edge(_, Node(k)) }).
    ";

    let (domain, mut session) = create_session(rules7x, MODEL1);

    let edge34 = session.parse_term_str("Edge(Node(3), Node(4))").unwrap();
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))").unwrap();

    session.add_term(edge56.clone());

    session.remove_terms(vec![edge34, edge56]);
}

#[test]
fn test_ddengine_8() {
    let rules8 = "
        Edge(x.src, d) :- x is Edge(a, b), y is Edge(b, c), Edge(y.dst, d).
    ";

    let (domain, mut session) = create_session(rules8, MODEL1);
}

#[test]
fn test_ddengine_8x() {
    let rules8x = "
        Edge(a, c) :- x is Edge(a, y.src), y is Edge(x.dst, c).
    ";

    let (domain, mut session) = create_session(rules8x, MODEL1);
}

#[test]
// TODO: Fix the constraint classification in rule.
fn test_ddengine_9() {
    // Let's try a nested aggregation.
    let rules9 = "
    TwoEdge(x, x, num) :- x is Edge(c, d), aggr1 = count({ n | n is Node(_), aggr2 = count({ x | x is Edge(a, b) }) }), num = aggr1 * 100 .
    ";

    let (domain, mut session) = create_session(rules9, MODEL1);
}


#[test]
fn test_ddengine_10() {
    let rules10 = "
    Node(num) :- num = aggr * 1000, aggr = count({Edge(a, a), b | Edge(a, b)}).
    ";

    let (domain, mut session) = create_session(rules10, MODEL1);
}


#[test]
fn test_ddengine_on_social_network() {
    let path = Path::new("./tests/samples/SocialNetwork.4ml");
    let content = fs::read_to_string(path).unwrap();
    

    let mut engine = DDEngine::new();
    engine.inspect = true;

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(content);
    // println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("SocialNetwork".to_string()).unwrap().clone();
    let model = engine.get_model("example".to_string()).unwrap().clone();

    let mut session = engine.create_session("SocialNetwork", Some("example"));
    session.load_model(model);
}
