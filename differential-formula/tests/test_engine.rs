extern crate differential_formula;

use differential_formula::constraint::*;
use differential_formula::expression::*;
use differential_formula::term::*;
use differential_formula::engine::*;
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

#[test]
fn test_ddengine() {

    let rules1 = "
        Path(a, b) :- Edge(a, b).
        Path(a, c) :- Path(a, b), Path(b, c).
    ";

    let rules2 = "
        Edge(a, c) :- Edge(a, b), Edge(b, c).
    ";

    let rules3 = "
        Edge(a, d) :- Edge(a, b), Edge(b, c), Edge(c, d).
    ";

    let rules4 = "
        Nocycle(u) :- Path(u, v), no Path(u, u).
    ";

    let rules5 = "
        Line(a, b, c, d) :- Edge(a, b), Edge(c, d).
    ";

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

    let model1 = "
    model m of Graph {
        n0 is Node(0).
        n1 is Node(1).
        n2 is Node(2).
        n3 is Node(3).
        n4 is Node(4).

        Edge(n0, n0).
        Edge(n0, n1).
        Edge(n1, n2).
        Edge(n2, n3).
        Edge(n3, n4).
    }";

    let program1 = generate_graph_program(rules7, model1);
    println!("{}", program1);

    let mut engine = DDEngine::new();

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(program1);
    println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("Graph".to_string()).unwrap();
    let model = engine.get_model("m".to_string()).unwrap();

    let mut session = engine.create_session("Graph".to_string());
    session.load_model(model);
}

#[test]
fn test_ddengine_2() {
    //let content = std::env::current_dir();
    let path = Path::new("./tests/samples/SocialNetwork.4ml");
    let content = fs::read_to_string(path).unwrap();
    

    let mut engine = DDEngine::new();

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(content);
    println!("{:?}", env);
    engine.install(env);

    let domain = engine.get_domain("SocialNetwork".to_string()).unwrap();
    let model = engine.get_model("example".to_string()).unwrap();

    let mut session = engine.create_session("SocialNetwork".to_string());
    session.load_model(model);
}