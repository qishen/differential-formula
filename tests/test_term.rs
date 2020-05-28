#![type_length_limit="1120927"]
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


fn parse_program(program: &str, model_name: &str) -> (Model, DDEngine) {
    let mut engine = DDEngine::new();
    engine.inspect = true;
    engine.install(program.to_string());
    let model = engine.env.get_model_by_name(model_name).unwrap().clone();
    (model, engine)
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

    let (model, engine) = parse_program(program, "g");
    let mut session = Session::new(model.clone(), &engine);

    let n1 = model.get_term_by_name("n1");
    let n1x = model.get_term_by_name("n1x");
    let n2 = model.get_term_by_name("n2");
    let n2x = model.get_term_by_name("n2x");
    let n3 = model.get_term_by_name("n3");
    let e1 = model.get_term_by_name("e1");
    let e1x = model.get_term_by_name("e1x");
    let e2 = model.get_term_by_name("e2");
    let e2x = model.get_term_by_name("e2x");
    let te1 = model.get_term_by_name("te1");

    let nv1 = model.get_term_by_name("nv1");
    let ev1 = model.get_term_by_name("ev1");
    let ev2 = model.get_term_by_name("ev2");
    let ev3 = model.get_term_by_name("ev3");
    let tev1 = model.get_term_by_name("tev1");


    println!("// -------- Test Term Equality -------- //");
    assert_eq!(n1, n1x);
    assert_eq!(e1, e1x);
    //assert_eq!(e2, e2x);

    // Without alias.
    let mut n1y = session.create_term("Node(1)").unwrap();
    assert_eq!(n1, &n1y);

    // Add random alias to `n1y` and see if it's still equal to `n1`.
    if let Term::Composite(c) = &mut n1y {
        c.alias = Some("hello".to_string());
    }
    assert_eq!(n1, &n1y);
    println!("{} is equal to {}", n1, n1y);

    let v1 = session.create_term("x").unwrap();
    let v1x = session.create_term("x").unwrap();
    let v2 = session.create_term("x.y.z").unwrap(); 
    let v2x = session.create_term("x.y.z").unwrap();

    // The root is a term without fragments.
    assert_eq!(v2.root(), v1.borrow());

    // Variable terms with same root but different fragments.
    assert_ne!(v1, v2);


    println!("// -------- Test Term Replacement -------- //");
    
    let x = session.create_term("x").unwrap();
    let y = session.create_term("y").unwrap();
    let xy = session.create_term("x.y").unwrap();

    // Create a copy and replace variable `x` and `y` with terms.
    let e_x_y = ev1.clone();
    let mut e_n1_y = e_x_y.clone();
    e_n1_y.replace(&x, n1);
    println!("Replace {} in {} with {} and finally get {}", x, e_x_y, n1, e_n1_y);
    let mut e_n1_n2 = e_n1_y.clone();
    e_n1_n2.replace(&y, n2);
    println!("Replace {} in {} with {} and finally get {}", y, e_n1_y, n2, e_n1_n2);
    assert_eq!(&e_n1_n2, e1);


    println!("// -------- Test Term Bindings -------- //");

    // Node(x) -> Node(1)
    let binding1 = nv1.get_bindings(n1).unwrap();
    // Edge(x, y) -> Edge(n1, n2)
    let mut binding2 = ev1.get_bindings(e1).unwrap();
    // Edge(Node(a), Node(b)) -> Edge(Node(1), Node(2))
    let binding3 = ev2.get_bindings(e1).unwrap();
    // Edge(_, Node(b)) -> Edge(n1, n2)
    let binding4 = ev3.get_bindings(e1).unwrap();
    // TwoEdge(x, y) -> TwoEdge(Edge(n1, n2), Edge(n2, n3))
    let mut binding5 = tev1.get_bindings(te1).unwrap();
    // 
    let edge_uu = session.create_term("Edge(u, u)").unwrap();
    let binding6 = edge_uu.get_bindings(e1);
    assert_eq!(binding6.clone(), None);

    let new_n1 = nv1.propagate_bindings(&binding1);
    let new_e1 = ev1.propagate_bindings(&binding2);

    assert_eq!(n1, new_n1.borrow());
    assert_eq!(e1, new_e1.borrow());

    println!("Bind {} to {} and get {:?}", nv1, n1, binding1);
    println!("Bind {} to {} and get {:?}", ev1, e1, binding2);
    println!("Bind {} to {} and get {:?}", ev2, e1, binding3);
    println!("Bind {} to {} and get {:?}", ev3, e1, binding4);
    println!("Bind {} to {} and get {:?}", tev1, te1, binding5);
    println!("Bind {} to {} and get {:?}", e1, edge_uu, binding6);


    println!("// -------- Test Extension on Term Bindings -------- //");
    
    // Testing on extension of bindings when the bindings has key `x` but wants to extend
    // the binding with key `x.y` or `x.y.z` which represent the subterms derived from the 
    // term `x` points to.
    let var = session.create_term("x.name").unwrap();
    println!("Original binding: {:?}", binding2);

    Term::update_binding(&Arc::new(var.clone()), &mut binding2);
    let atom1 = session.create_term("1").unwrap();

    assert_eq!(binding2.get(&var).unwrap(), &atom1);
    println!("Updated binding: {:?}", binding2);

    let var1 = session.create_term("x.src").unwrap();
    let varx = session.create_term("x").unwrap();
    let var2 = session.create_term("x.src.name").unwrap();
    println!("Original binding: {:?}", binding5);

    Term::update_binding(&Arc::new(var1.clone()), &mut binding5);
    binding5.remove(&Arc::new(varx));

    Term::update_binding(&Arc::new(var2), &mut binding5);
    println!("Updated binding {:?}", binding5);
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

    let (model, engine) = parse_program(program, "g");
    let session = Session::new(model.clone(), &engine);

    let n1 = model.get_term_by_name("n1");
    let n2 = model.get_term_by_name("n2");
    let x1 = model.get_term_by_name("x1");
    let x2 = model.get_term_by_name("x2");

    let te1 = model.get_term_by_name("te1");
    let te2 = model.get_term_by_name("te2");

    let v0 = session.create_term("x.one").unwrap();
    let v0x = session.create_term("x.wrong").unwrap();
    let v1 = session.create_term("x.one.src").unwrap();
    let v2 = session.create_term("y.two.dst").unwrap();
    let v2x = session.create_term("y.two.dst.wrong").unwrap();

    println!("// -------- Test Finding Subterm -------- //");

    let subterm1_arc = te1.find_subterm(&v0).unwrap();
    let subterm1x_arc = te1.find_subterm_by_labels(&vec!["one".to_string()]).unwrap();
    let subterm1: &Term = subterm1_arc.borrow();
    let subterm1x: &Term = subterm1x_arc.borrow();

    assert_eq!(subterm1, x1);
    println!("Use {} to find subterm in {} and the result is {}", v0, te1, subterm1);

    assert_eq!(subterm1x, x1);
    println!("Use {:?} to find subterm in {} and the result is {}", vec!["one"], te1, subterm1x);

    let subterm2_arc = te1.find_subterm(&v1).unwrap();
    let subterm2: &Term = subterm2_arc.borrow();
    println!("Use {} to find subterm in {} and the result is {}", v1, te1, subterm2);
    assert_eq!(subterm2, n1);

    let subterm3_arc = te2.find_subterm(&v2).unwrap();
    let subterm3: &Term = subterm3_arc.borrow();
    println!("Use {} to find subterm in {} and the result is {}", v2, te2, subterm3);
    assert_eq!(subterm3, n2);

    // Given unmatched label at the beginning.
    let subterm4_arc = te1.find_subterm(&v0x);
    println!("Use {} to find subterm in {} and the result is {:?}", v0x, te1, subterm4_arc);
    assert_eq!(subterm4_arc, None);

    // Given unmatched label at the end.
    let subterm5_arc = te1.find_subterm(&v2x);
    println!("Use {} to find subterm in {} and the result is {:?}", v2x, te1, subterm5_arc);
    assert_eq!(subterm5_arc, None);

    assert_eq!(v0.has_subterm(&v0), Some(true));
    assert_eq!(v0.has_subterm(&v1), Some(true));
    assert_eq!(v1.has_subterm(&v0), Some(false));
    assert_eq!(v0x.has_subterm(&v2x), Some(false));

    // Only works on comparison between variable terms.
    assert_eq!(v0.has_subterm(&n1), None);
    assert_eq!(n1.has_subterm(&v0), None);
}