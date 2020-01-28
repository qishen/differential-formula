extern crate differential_formula;

use differential_formula::engine::*;
use differential_formula::type_system::*;
use differential_formula::term::{Term, Composite, Variable, Atom, TermBehavior};

use std::borrow::Borrow;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn parse_program(program: &str, model_name: &str) -> Model {
    let mut engine = DDEngine::new();
    engine.inspect = true;

    let env = DDEngine::parse_string(program.to_string());
    //  println!("{:?}", env);
    engine.install(env);
    let model = engine.get_model(model_name.to_string()).unwrap().clone();
    model
}


#[test]
fn test_term_equality() {
    let program = "
        domain Graph {
            Node ::= new (name: Integer).
            Edge ::= new (src: Node, dst: Node).
        }

        model g of Graph {
            n1 is Node(1).
            n1x is Node(1).
            n2 is Node(2).
            n3 is Node(3).
            e1 is Edge(n1, n2).
            e1x is Edge(n1, n2).
            e2 is Edge(n2, n3).
        }
    ";

    let model = parse_program(program, "g");
    let n1 = model.get_term_by_name("n1");
    let n1x = model.get_term_by_name("n1x");
    let e1 = model.get_term_by_name("e1");
    let e1x = model.get_term_by_name("e1x");
    let e2 = model.get_term_by_name("e2");
    let v1: Term = Variable::new("x".to_string(), vec![]).into();
    let v1x: Term = Variable::new("x".to_string(), vec![]).into();
    let v2: Term = Variable::new("x".to_string(), vec!["y".to_string(), "z".to_string()]).into();
    let v2x: Term = Variable::new("x".to_string(), vec!["y".to_string(), "z".to_string()]).into();
    
    // same terms with different alias are still equal.
    assert_eq!(n1, n1x);
    assert_eq!(e1, e1x);
    assert_eq!(v1, v1x);
    assert_eq!(v2, v2x);

    assert_eq!(v2.root(), &v1);
    // variable terms with same root but different fragments.
    assert_ne!(v1, v2);

    println!("{}", n1);
    println!("{}", n1x);
    println!("{}", e1);
    println!("{}", e2);
}

#[test]
fn test_term_bindings() {
    let program = "
        domain Graph {
            Node ::= new (name: Integer).
            Edge ::= new (src: Node, dst: Node).
        }

        model g of Graph {
            n1 is Node(1).
            n2 is Node(2).
            n3 is Node(3).
            e1 is Edge(n1, n2).
            e2 is Edge(n2, n3).

            nv1 is Node(x).
            ev1 is Edge(x, y).
            ev2 is Edge(Node(a), Node(b)).
            ev3 is Edge(_, Node(b)).
        }
    ";

    let model = parse_program(program, "g");
    let n1 = model.get_term_by_name("n1");
    let nv1 = model.get_term_by_name("nv1");
    let e1 = model.get_term_by_name("e1");
    let ev1 = model.get_term_by_name("ev1");
    let ev2 = model.get_term_by_name("ev2");
    let ev3 = model.get_term_by_name("ev3");
    
    // Node(x) -> Node(1)
    let binding1 = nv1.get_bindings(&Arc::new(n1.clone())).unwrap();
    // Edge(x, y) -> Edge(n1, n2)
    let binding2 = ev1.get_bindings(&Arc::new(e1.clone())).unwrap();
    // Edge(Node(a), Node(b)) -> Edge(Node(1), Node(2))
    let binding3 = ev2.get_bindings(&Arc::new(e1.clone())).unwrap();
    // Edge(_, Node(b)) -> Edge(n1, n2)
    let binding4 = ev3.get_bindings(&Arc::new(e1.clone())).unwrap();

    let new_n1 = nv1.propagate_bindings(&binding1).unwrap();
    let new_e1 = ev1.propagate_bindings(&binding2).unwrap();

    assert_eq!(n1, new_n1.borrow());
    assert_eq!(e1, new_e1.borrow());

    println!("{:?}", binding1);
    println!("{:?}", binding2);
    println!("{:?}", binding3);
    println!("{:?}", binding4);
    
    println!("{:?}", new_n1);
    println!("{:?}", new_e1);
}

#[test]
fn test_subterm() {
    let program = "
        domain Graph {
            Node ::= new (name: Integer).
            Edge ::= new (src: Node, dst: Node).
            TwoEdge ::= new (one: Edge, two: Edge).
        }

        model g of Graph {
            n1 is Node(1).
            n2 is Node(2).
            n3 is Node(3).
            x1 is Edge(n1, n2).
            x2 is Edge(n2, n3).
            te1 is TwoEdge(x1, x2).
            te2 is TwoEdge(x2, x1).
        }
    ";

    let model = parse_program(program, "g");
    let n1 = model.get_term_by_name("n1");
    let n2 = model.get_term_by_name("n2");
    let x1 = model.get_term_by_name("x1");
    let x2 = model.get_term_by_name("x2");

    let te1 = model.get_term_by_name("te1");
    let te2 = model.get_term_by_name("te2");
    let v0: Term = Variable::new("x".to_string(), vec!["one".to_string()]).into();
    let v0x: Term = Variable::new("x".to_string(), vec!["wrong".to_string()]).into();
    let v1: Term = Variable::new("x".to_string(), vec!["one".to_string(), "src".to_string()]).into();
    let v2: Term = Variable::new("y".to_string(), vec!["two".to_string(), "dst".to_string()]).into();
    let v2x: Term = Variable::new("y".to_string(), vec!["two".to_string(), "dst".to_string(), "wrong".to_string()]).into();

    let subterm1_arc = Term::find_subterm(Arc::new(te1.clone()), &v0).unwrap();
    let subterm1: &Term = subterm1_arc.borrow();
    assert_eq!(subterm1, x1);

    let subterm2_arc = Term::find_subterm(Arc::new(te1.clone()), &v1).unwrap();
    let subterm2: &Term = subterm2_arc.borrow();
    assert_eq!(subterm2, n1);

    let subterm3_arc = Term::find_subterm(Arc::new(te2.clone()), &v2).unwrap();
    let subterm3: &Term = subterm3_arc.borrow();
    assert_eq!(subterm3, n2);

    // Given unmatched label at the beginning.
    let subterm4_arc = Term::find_subterm(Arc::new(te1.clone()), &v0x);
    assert_eq!(subterm4_arc, None);

    // Given unmatched label at the end.
    let subterm5_arc = Term::find_subterm(Arc::new(te1.clone()), &v2x);
    assert_eq!(subterm5_arc, None);

    assert_eq!(v0.is_subterm(&v1), Some(true));
    assert_eq!(v1.is_subterm(&v0), Some(false));
    assert_eq!(v0x.is_subterm(&v2x), Some(false));
    // Only works on comparison between variable terms.
    assert_eq!(v0.is_subterm(&n1), None);
    assert_eq!(n1.is_subterm(&v0), None);
}