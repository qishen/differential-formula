// #![type_length_limit="1120927"]
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use differential_formula::engine::*;
use differential_formula::expression::*;
use differential_formula::module::*;
use differential_formula::term::*;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn parse_program(program: &str, model_name: &str) -> (Model<AtomicTerm>, DDEngine<AtomicTerm>) {
    let env = AtomicTerm::load_program(program.to_string());
    println!("{:#?}", env);
    let mut engine = DDEngine::new(env);
    engine.inspect = true;
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
            n1 is Node(1). // n1x is Node(1).
            n2 is Node(2). // n2x is Node(2).
            n3 is Node(3).

            e1 is Edge(n1, n2).
            // e1x is Edge(n1, n2).
            e2 is Edge(n2, n3).
            //e2x is Edge(n2, Node(3)).
            te1 is TwoEdge(Edge(n1, n2), Edge(n2, n3)).

            // Terms that contain variables inside.
            nv1 is Node(x).
            ev1 is Edge(x, y).
            ev2 is Edge(Node(a), Node(b)).
            ev3 is Edge(_, Node(b)).
            ev4 is Edge(y, z).
            ev5 is Edge(a, a).
            tev1 is TwoEdge(x, y).

            // Terms for testing normalization
        }
    ";

    let (model, engine) = parse_program(program, "g");
    let session = Session::new(model.clone(), &engine);
    
    let name = AtomicTerm::create_variable_term(None, "n1".to_string(), vec![]);
    let node1 = model.model_store().get_term_by_alias(&"n1".into());
    // TODO: It's a bi-directional map and can't have two alias point to the same term.
    // let node1x = model.model_store().get_term_by_alias(&"n1x".into());
    let node2 = model.model_store().get_term_by_alias(&"n2".into());
    // let node2x = model.model_store().get_term_by_alias(&"n2x".into());
    let node3 = model.model_store().get_term_by_alias(&"n3".into());
    let edge_n1_n2 = model.model_store().get_term_by_alias(&"e1".into());
    // let edge_n1_n2x = model.model_store().get_term_by_alias(&"e1x".into());
    let edge_n2_n3 = model.model_store().get_term_by_alias(&"e2".into());
    // let edge_n2_n3x = model.model_store().get_term_by_alias(&"e2x".into());
    let two_edge_e1_e2 = model.model_store().get_term_by_alias(&"te1".into());

    let node_x = model.model_store().get_term_by_alias(&"nv1".into());
    let edge_x_y = model.model_store().get_term_by_alias(&"ev1".into());
    let edge_na_nb = model.model_store().get_term_by_alias(&"ev2".into());
    let edge_x_nb = model.model_store().get_term_by_alias(&"ev3".into());
    let edge_y_z = model.model_store().get_term_by_alias(&"ev4".into());
    let edge_a_a = model.model_store().get_term_by_alias(&"ev5".into());
    let two_edge_x_y = model.model_store().get_term_by_alias(&"tev1".into());


    println!("// -------- Test Term Equality -------- //");
    // assert_eq!(node1, node1x);
    // assert_eq!(edge_n1_n2, edge_n1_n2x);
    // assert_eq!(e2, e2x);

    // Without alias.
    let mut node1_no_alias = session.create_term("Node(1)").unwrap();
    assert_eq!(node1, &node1_no_alias);

    // Add random alias to `n1y` and see if it's still equal to `n1`.
    if let AtomicTerm::Composite(c) = &mut node1_no_alias {
        c.alias = Some("hello".to_string());
    }
    assert_eq!(node1, &node1_no_alias);
    println!("{} is equal to {}", node1, node1_no_alias);

    let x = session.create_term("x").unwrap();
    let xyz = session.create_term("x.y.z").unwrap(); 

    // The root is a term without fragments.
    assert_eq!(xyz.root(), &x);

    // Variable terms with same root but different fragments.
    assert_ne!(x, xyz);


    println!("// -------- Test Term Replacement -------- //");
    
    let x = session.create_term("x").unwrap();
    let y = session.create_term("y").unwrap();
    let xy = session.create_term("x.y").unwrap();

    // Create a copy and replace variable `x` and `y` with terms.
    let e_x_y = edge_n1_n2.clone();
    let mut e_n1_y = e_x_y.clone();
    e_n1_y.replace_pattern(&x, node1);

    println!("Replace {} in {} with {} and finally get {}", x, e_x_y, node1, e_n1_y);
    let mut e_n1_n2 = e_n1_y.clone();
    
    println!("Replace {} in {} with {} and finally get {}", y, e_n1_y, node2, e_n1_n2);
    assert_eq!(&e_n1_n2, edge_n1_n2);


    println!("// -------- Test Term Bindings -------- //");
    // Node(x) -> Node(1)
    let binding1 = node_x.get_bindings(node1).unwrap();
    // Edge(x, y) -> Edge(n1, n2)
    let mut binding2 = edge_x_y.get_bindings(edge_n1_n2).unwrap();
    // Edge(Node(a), Node(b)) -> Edge(Node(1), Node(2))
    let binding3 = edge_na_nb.get_bindings(edge_n1_n2).unwrap();
    // Edge(_, Node(b)) -> Edge(n1, n2)
    let binding4 = edge_x_nb.get_bindings(edge_n1_n2).unwrap();
    // TwoEdge(x, y) -> TwoEdge(Edge(n1, n2), Edge(n2, n3))
    let mut binding5 = two_edge_x_y.get_bindings(two_edge_e1_e2).unwrap();
    // Edge(u, u) -> Edge(n1, n2).
    let edge_uu = session.create_term("Edge(u, u)").unwrap();
    let binding6 = edge_uu.get_bindings(edge_n1_n2);
    assert_eq!(binding6.clone(), None);

    let new_n1 = node_x.propagate_bindings(&binding1);
    let new_e1 = edge_x_y.propagate_bindings(&binding2);

    assert_eq!(node1, &new_n1);
    assert_eq!(edge_n1_n2, &new_e1);

    println!("Bind {} to {} and get {:?}", node_x, node1, binding1);
    println!("Bind {} to {} and get {:?}", edge_x_y, edge_n1_n2, binding2);
    println!("Bind {} to {} and get {:?}", edge_na_nb, edge_n1_n2, binding3);
    println!("Bind {} to {} and get {:?}", edge_x_nb, edge_n1_n2, binding4);
    println!("Bind {} to {} and get {:?}", two_edge_x_y, two_edge_e1_e2, binding5);
    println!("Bind {} to {} and get {:?}", edge_uu, edge_n1_n2, binding6);


    println!("// -------- Test Extension on Term Bindings -------- //");
    // Testing on extension of bindings when the bindings has key `x` but wants to extend
    // the binding with key `x.y` or `x.y.z` which represent the subterms derived from the 
    // term `x` points to.
    let var = session.create_term("x.name").unwrap();
    println!("Original binding: {:?}", binding2);

    var.update_binding(&mut binding2);
    let atom1 = session.create_term("1").unwrap();

    assert_eq!(binding2.get(&var).unwrap(), &atom1);
    println!("Updated binding: {:?}", binding2);

    let var1 = session.create_term("x.src").unwrap();
    let varx = session.create_term("x").unwrap();
    let var2 = session.create_term("x.src.name").unwrap();
    println!("Original binding: {:?}", binding5);

    var1.update_binding(&mut binding5);
    binding5.remove(&Arc::new(varx));

    var2.update_binding(&mut binding5);
    println!("Updated binding {:?}", binding5);

    println!("// -------- Test Term Normalization -------- //");
    let normalized_ev1 = edge_x_y.normalize().0;
    let normalized_ev4 = edge_y_z.normalize().0;
    let normalized_ev5 = edge_a_a.normalize().0;
    println!("{} is normalized to {}", edge_x_y, normalized_ev1);
    println!("{} is normalized to {}", edge_y_z, normalized_ev4);
    println!("{} is normalized to {}", edge_a_a, normalized_ev5);
    assert_eq!(normalized_ev1, normalized_ev4);
    assert_ne!(normalized_ev4, normalized_ev5);
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

    let n1 = model.model_store().get_term_by_alias(&"n1".into());
    let n2 = model.model_store().get_term_by_alias(&"n2".into());
    let x1 = model.model_store().get_term_by_alias(&"x1".into());
    let x2 = model.model_store().get_term_by_alias(&"x2".into());

    let te1 = model.model_store().get_term_by_alias(&"te1".into());
    let te2 = model.model_store().get_term_by_alias(&"te2".into());

    let x_one = session.create_term("x.one").unwrap();
    let x_wrong = session.create_term("x.wrong").unwrap();
    let x_one_src = session.create_term("x.one.src").unwrap(); //v1
    let y_two_dst = session.create_term("y.two.dst").unwrap(); //v2
    let y_two_dst_wrong = session.create_term("y.two.dst.wrong").unwrap();

    println!("// -------- Test Finding Subterm -------- //");
    let subterm1 = te1.find_subterm(&x_one).unwrap();
    let subterm1x = te1.find_subterm_by_labels(&vec![&"one".to_string()]).unwrap();

    assert_eq!(subterm1, x1);
    println!("Use {} to find subterm in {} and the result is {}", x_one, te1, subterm1);

    assert_eq!(subterm1x, x1);
    println!("Use {:?} to find subterm in {} and the result is {}", vec!["one"], te1, subterm1x);

    let subterm2 = te1.find_subterm(&x_one_src).unwrap();
    println!("Use {} to find subterm in {} and the result is {}", x_one_src, te1, subterm2);
    assert_eq!(subterm2, n1);

    let subterm3 = te2.find_subterm(&y_two_dst).unwrap();
    println!("Use {} to find subterm in {} and the result is {}", y_two_dst, te2, subterm3);
    assert_eq!(subterm3, n2);

    // Given unmatched label at the beginning.
    let subterm4 = te1.find_subterm(&x_wrong);
    println!("Use {} to find subterm in {} and the result is {:?}", x_wrong, te1, subterm4);
    assert_eq!(subterm4, None);

    // Given unmatched label at the end.
    let subterm5 = te1.find_subterm(&y_two_dst_wrong);
    println!("Use {} to find subterm in {} and the result is {:?}", y_two_dst_wrong, te1, subterm5);
    assert_eq!(subterm5, None);

    assert_eq!(x_one.is_direct_subterm_of(&x_one), true);
    assert_eq!(x_one_src.is_direct_subterm_of(&x_one), true);
    assert_eq!(x_one.is_direct_subterm_of(&x_one_src), false);
    assert_eq!(y_two_dst_wrong.is_direct_subterm_of(&x_wrong), false);
}