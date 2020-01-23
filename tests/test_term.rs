extern crate differential_formula;

use differential_formula::engine::*;
use differential_formula::type_system::*;
use differential_formula::term::{Term, Composite, Variable, Atom, TermBehavior};
use differential_formula::composite;
use differential_formula::variable;
use differential_formula::atom;

use std::borrow::Borrow;
use std::sync::Arc;
use std::rc::Rc;
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


/*
fn generate_small_graph_terms() -> (Term, Term, Term, Term, Term, Term, Term){
    let hello: Term = Atom::Str("hello".to_string()).into();
    let world: Term = Atom::Str("world".to_string()).into();
    let n1: Term = composite!{ n1 is Node => (hello.clone())}.into();
    let n2: Term = composite!{ n2 is Node(world.clone())}.into();
    let vn1: Term = variable!(x.);
    let vn2: Term = variable!(y.);
    // Edge(Node("hello"), Node("world"))
    let e1: Term = composite!{ e1 is Edge(n1.clone(), n2.clone())};
    // Edge(x, Node("world"))
    let e2: Term = composite!{ e2 is Edge(vn1.clone(), n2.clone())};
    // Edge(x, y)
    let e3: Term = composite!{ e3 is Edge(vn1.clone(), vn2.clone())};
    
    (n1, n2, vn1, vn2, e1, e2, e3)
}

fn generate_simple_terms() -> (Term, Term, Term, Term, Term, Term) {
    let atom1: Term = Atom::Int(1).into();
    let atom1_copy: Term = Atom::Int(1).into();
    let atom2: Term = Atom::Int(2).into();
    let var1: Term = variable!(x.y.z);
    let c1: Term = composite!{ e1 is E(atom1.clone(), var1.clone())};
    let c1_copy: Term = composite!{ e1 is E(atom1.clone(), var1.clone())};

    (atom1, atom1_copy, atom2, var1, c1, c1_copy)
}

#[test]
fn test_implicit_variable_constraints() {
    let hello: Term = Atom::Str("hello".to_string()).into();
    let world: Term = Atom::Str("world".to_string()).into();
    let node_hello: Term = composite! { Node(hello) };
    let node_world: Term = composite! { Node(world) };
    let edge_ab: Term = composite! { Edge(variable!(a.), variable!(b.)) }.into();
    let edge_uu: Term = composite! { Edge(variable!(u.), variable!(u.)) }.into();

    let e1: Term = composite! { Edge(node_hello.clone(), node_world.clone()) }.into();
    let e2: Term = composite! { Edge(node_hello.clone(), node_hello.clone()) }.into();

    let binding1 = edge_ab.get_bindings(&e1);
    let binding2 = edge_uu.get_bindings(&e1);
    
    assert_eq!(binding2, None);
    
}

#[test]
fn test_term_equality() {
    let (atom1, atom1_copy, atom2, var1, c1, c1_copy) = generate_simple_terms();
    assert_ne!(atom1, atom2);
    assert_eq!(atom1 == atom1_copy, true);
    assert_eq!(c1 == c1_copy, true);
    assert_eq!(vec![c1.clone(), atom1.clone()] == vec![c1_copy.clone(), atom1.clone()], true);
    assert_eq!(atom1 == c1, false);
    assert_eq!(calculate_hash(&atom1), calculate_hash(&atom1_copy));
    assert_eq!(calculate_hash(&c1), calculate_hash(&c1_copy));
    println!("\n{}", atom1);
    println!("\n{}", var1);
    println!("\n{}", c1);
}

#[test]
fn test_term_methods() {
    let (n1, n2, vn1, vn2, e1, e2, e3) = generate_small_graph_terms();
    let map1 = e2.get_bindings(&e1).unwrap();
    let map2 = e3.get_bindings(&e1).unwrap();
    // test variables()
    assert_eq!(e2.variables().len(), 1);
    assert_eq!(e3.variables().len(), 2);
    // test is_groundterm()
    assert_eq!(e1.is_groundterm(), true);
    assert_eq!(e2.is_groundterm(), false);
    // test get_bindings()
    assert_eq!(map1.len(), 1);
    assert_eq!(map2.len(), 2);
    // test propagate_bindings()
    let e2x = e2.propagate_bindings(&map1);
    let e3x = e3.propagate_bindings(&map2);
    assert_eq!(e2x.is_groundterm(), true);
    assert_eq!(e3x.is_groundterm(), true);
}
*/