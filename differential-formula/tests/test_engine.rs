extern crate differential_formula;

use differential_formula::constraint::*;
use differential_formula::expression::*;
use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::type_system::*;
use differential_formula::rule::*;

use differential_formula::composite;
use differential_formula::variable;
use differential_formula::atom;

use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


static model1: &str = "
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

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(program);
    //  println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("Graph".to_string()).unwrap();
    let model = engine.get_model("m".to_string()).unwrap();

    let mut session = engine.create_session("Graph".to_string());
    session.load_model(model);
    (domain, session)
}

#[test]
fn test_ddengine_1() {
    let rules1 = "
        Path(a, b) :- Edge(a, b).
        //Path(a, c) :- Path(a, b), Path(b, c).
    ";

    let (domain, mut session) = create_session(rules1, model1);
}

#[test]
fn test_ddengine_2() {
    let rules2 = "
        Edge(a, c) :- Edge(a, b), Edge(b, c).
    ";
    
    let (domain, mut session) = create_session(rules2, model1);
    
    let edge45 = session.parse_term_str("Edge(Node(4), Node(5))".to_string());
    let edge56 = session.parse_term_str("Edge(Node(5), Node(6))".to_string());

    session.add_terms(vec![edge45, edge56]);
}

#[test]
fn test_ddengine_3() {
    let rules3 = "
        Edge(a, d) :- Edge(a, b), Edge(b, c), Edge(c, d).
    ";

    let (domain, mut session) = create_session(rules3, model1);
}

#[test]
fn test_ddengine_4() {
    let rules4 = "
        Path(a, b) :- Edge(a, b).
        Path(a, c) :- Path(a, b), Path(b, c).
        Nocycle(u) :- u is Node(_), no Path(u, u).
    ";

    let (domain, mut session) = create_session(rules4, model1);

    let edge00 = session.parse_term_str("Edge(Node(0), Node(0))".to_string());
    let edge22 = session.parse_term_str("Edge(Node(2), Node(2))".to_string());
    let edge45 = session.parse_term_str("Edge(Node(5), Node(5))".to_string());
    
    session.add_term(edge45);
    session.remove_term(edge00);
}

#[test]
fn test_ddengine_5() {
    let rules5 = "
        Line(a, b, c, d) :- Edge(a, b), Edge(c, d).
    ";

    let (domain, mut session) = create_session(rules5, model1);
}

fn test_ddengine() {

    let rules6 = "
        TwoEdge(x, y) :- x is Edge(a, b), y is Edge(b, c).
    ";

    let rules6x = "
        TwoEdge(x, y) :- x is Edge(_, _), y is Edge(_, _).
    ";

    let rules7 = "
        TwoEdge(x, x, square) :- x is Edge(c, d), 
                                 aggr = count({Edge(a, a), b | Edge(a, b)}), 
                                 square = aggr * aggr, aggr * 2 > 16 .
    ";

    let rules8 = "
        Edge(x.src, d) :- x is Edge(a, b), y is Edge(b, c), Edge(y.dst, d).
    ";

    let rules8x = "
        Edge(a, c) :- x is Edge(a, y.src), y is Edge(x.dst, c).
    ";

}


#[test]
fn test_ddengine_on_social_network() {
    //let content = std::env::current_dir();
    let path = Path::new("./tests/samples/SocialNetwork.4ml");
    let content = fs::read_to_string(path).unwrap();
    

    let mut engine = DDEngine::new();

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(content);
    // println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("SocialNetwork".to_string()).unwrap();
    let model = engine.get_model("example".to_string()).unwrap();

    let mut session = engine.create_session("SocialNetwork".to_string());
    session.load_model(model);
}
