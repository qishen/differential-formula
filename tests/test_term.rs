extern crate differential_formula;

use std::borrow::Borrow;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

use differential_formula::engine::*;
use differential_formula::expression::*;
use differential_formula::module::*;
use differential_formula::type_system::*;
use differential_formula::term::*;
use num::*;


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn parse_program(program: &str, model_name: &str) -> Model {
    let mut engine = DDEngine::new();
    engine.inspect = true;
    engine.install(program.to_string());
    let model = engine.env.get_model_by_name(model_name).unwrap().clone();
    model
}

fn create_term_map(model: Model, names: Vec<&str>) -> HashMap<String, Term> {
    let mut term_map = HashMap::new();
    for name in names {
        let term = model.get_term_by_name(name);
        term_map.insert(name.to_string(), term.clone());
    }
    term_map
}

#[test]
fn test_term_bindings() {
    let program = "
        domain Graph {
            Node ::= new (name: Integer).
            Edge ::= new (src: Node, dst: Node).
            TwoEdge ::= new (one: Edge, two: Edge).
        }

        model g of Graph {
            n1 is Node(1). n1x is Node(1).
            n2 is Node(2). n2x is Node(2).
            n3 is Node(3).

            e1 is Edge(n1, n2).
            e1x is Edge(n1, n2).
            e2 is Edge(n2, n3).
            e2x is Edge(n2, Node(3)).
            te1 is TwoEdge(Edge(n1, n2), Edge(n2, n3)).

            // Terms that contain variables inside.
            nv1 is Node(x).
            ev1 is Edge(x, y).
            ev2 is Edge(Node(a), Node(b)).
            ev3 is Edge(_, Node(b)).
            tev1 is TwoEdge(x, y).
        }
    ";

    let model = parse_program(program, "g");
    let names = vec![
        "n1", "n1x", "n2", "n2x", "n3", "e1", "e1x", "e2", "e2x", "te1", 
        "nv1", "ev1", "ev2", "ev3", "tev1"
    ];

    let term_map = create_term_map(model, names);

    let n1 = term_map.get("n1").unwrap();
    let n1x = term_map.get("n1x").unwrap();
    let n2 = term_map.get("n2").unwrap();
    let n2x = term_map.get("n2x").unwrap();
    let n3 = term_map.get("n3").unwrap();
    let e1 = term_map.get("e1").unwrap();
    let e1x = term_map.get("e1x").unwrap();
    let e2 = term_map.get("e2").unwrap();
    let e2x = term_map.get("e2x").unwrap();
    let te1 = term_map.get("te1").unwrap();

    let nv1 = term_map.get("nv1").unwrap();
    let ev1 = term_map.get("ev1").unwrap();
    let ev2 = term_map.get("ev2").unwrap();
    let ev3 = term_map.get("ev3").unwrap();
    let tev1 = term_map.get("tev1").unwrap();


    // -------- Test Term Equality -------- //
    assert_eq!(n1, n1x);
    assert_eq!(e1, e1x);
    assert_eq!(e2, e2x);

    let v1: Term = Variable::new("x".to_string(), vec![]).into();
    let v1x: Term = Variable::new("x".to_string(), vec![]).into();
    let v2: Term = Variable::new("x".to_string(), vec!["y".to_string(), "z".to_string()]).into();
    let v2x: Term = Variable::new("x".to_string(), vec!["y".to_string(), "z".to_string()]).into();

    // The root is a term without fragments.
    assert_eq!(v2.root(), &v1);

    // Variable terms with same root but different fragments.
    assert_ne!(v1, v2);


    // -------- Test Term Replacement -------- //
    assert_eq!(nv1.match_with(n1), true);
    
    let x: Term = Variable::new("x".to_string(), vec![]).into();
    let y: Term = Variable::new("y".to_string(), vec![]).into();
    // Create a copy and replace variable `x` and `y` with terms.
    let mut ev1_copy = ev1.clone();
    ev1_copy.replace(&x, n1);
    ev1_copy.replace(&y, n2);
    assert_eq!(&ev1_copy, e1);


    // -------- Test Term Bindings -------- //

    // Node(x) -> Node(1)
    let binding1 = nv1.get_bindings(&Arc::new(n1.clone())).unwrap();
    // Edge(x, y) -> Edge(n1, n2)
    let mut binding2 = ev1.get_bindings(&Arc::new(e1.clone())).unwrap();
    // Edge(Node(a), Node(b)) -> Edge(Node(1), Node(2))
    let binding3 = ev2.get_bindings(&Arc::new(e1.clone())).unwrap();
    // Edge(_, Node(b)) -> Edge(n1, n2)
    let binding4 = ev3.get_bindings(&Arc::new(e1.clone())).unwrap();
    // TwoEdge(x, y) -> TwoEdge(Edge(n1, n2), Edge(n2, n3))
    let mut binding5 = tev1.get_bindings(&Arc::new(te1.clone())).unwrap();

    let new_n1 = nv1.propagate_bindings(&binding1).unwrap();
    let new_e1 = ev1.propagate_bindings(&binding2).unwrap();

    //assert_eq!(n1, new_n1.borrow());
    //assert_eq!(e1, new_e1.borrow());

    println!("{:?}", binding1);
    println!("{:?}", binding2);
    println!("{:?}", binding3);
    println!("{:?}", binding4);
    println!("{:?}", binding5);
    
    // Testing on extension of bindings.
    let var: Term = Variable::new("x".to_string(), vec!["name".to_string()]).into();
    Term::update_binding(&Arc::new(var.clone()), &mut binding2);
    let atom1: Term = Atom::Int(BigInt::from_i64(1).unwrap()).into();
    assert_eq!(binding2.get(&var).unwrap(), &Arc::new(atom1));
    println!("Updated binding {:?}", binding2);

    let var1: Term = Variable::new("x".to_string(), vec!["src".to_string()]).into();
    let varx: Term = Variable::new("x".to_string(), vec![]).into();
    let var2: Term = Variable::new("x".to_string(), vec!["src".to_string(), "name".to_string()]).into();
    Term::update_binding(&Arc::new(var1.clone()), &mut binding5);
    binding5.remove(&Arc::new(varx));
    Term::update_binding(&Arc::new(var2), &mut binding5);
    println!("Updated binding {:?}", binding5);
}

/*
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
    let n1 = term_map.get("n1");
    let n2 = term_map.get("n2");
    let x1 = term_map.get("x1");
    let x2 = term_map.get("x2");

    let te1 = term_map.get("te1");
    let te2 = term_map.get("te2");
    let v0: Term = Variable::new("x".to_string(), vec!["one".to_string()]).into();
    let v0x: Term = Variable::new("x".to_string(), vec!["wrong".to_string()]).into();
    let v1: Term = Variable::new("x".to_string(), vec!["one".to_string(), "src".to_string()]).into();
    let v2: Term = Variable::new("y".to_string(), vec!["two".to_string(), "dst".to_string()]).into();
    let v2x: Term = Variable::new("y".to_string(), vec!["two".to_string(), "dst".to_string(), "wrong".to_string()]).into();

    let subterm1_arc = Term::find_subterm(Arc::new(te1.clone()), &v0).unwrap();
    let subterm1x_arc = Term::find_subterm_by_labels(Arc::new(te1.clone()), &vec!["one".to_string()]).unwrap();
    let subterm1: &Term = subterm1_arc.borrow();
    let subterm1x: &Term = subterm1x_arc.borrow();
    assert_eq!(subterm1, x1);
    assert_eq!(subterm1x, x1);

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

    assert_eq!(v0.has_subterm(&v0), Some(true));
    assert_eq!(v0.has_subterm(&v1), Some(true));
    assert_eq!(v1.has_subterm(&v0), Some(false));
    assert_eq!(v0x.has_subterm(&v2x), Some(false));

    // Only works on comparison between variable terms.
    assert_eq!(v0.has_subterm(&n1), None);
    assert_eq!(n1.has_subterm(&v0), None);
}*/