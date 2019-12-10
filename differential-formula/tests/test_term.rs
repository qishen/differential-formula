extern crate differential_formula;

use differential_formula::term::{Term, Composite, Variable, Atom, TermBehavior};
use differential_formula::composite;
use differential_formula::variable;
use differential_formula::atom;

use std::sync::Arc;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


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
