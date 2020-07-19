#![feature(core_intrinsics)]
#![type_length_limit="1120927"]
extern crate rand;

use std::intrinsics::type_name;
use std::marker::*;
use std::hash::Hash;
use std::collections::*;
use rand::{Rng, SeedableRng, StdRng};
use num::*;
use serde::{Serialize, Deserialize};

use differential_dataflow::*;
use differential_dataflow::input::*;
use differential_dataflow::operators::Iterate;
use differential_dataflow::operators::Join;
use differential_dataflow::operators::Threshold;
use differential_dataflow::operators::arrange::*;
use differential_dataflow::operators::join::*;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::hashable::*;

use differential_formula::module::*;
use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::util::wrapper::*;
use differential_formula::util::map::*;
use differential_formula::parser::combinator::*;

use timely::dataflow::*;
use timely::dataflow::scopes::*;
use timely::worker::*;
use timely::communication::*;

#[macro_use]
extern crate lazy_static;

lazy_static!{
// Some statics that need to be initialized at runtime.
    static ref GRAPH_PROGRAM: String = "
        domain Graph {
            Node ::= new(name: Integer).
            Edge ::= new(src: Node, dst: Node).
            Path ::= new(src: Node, dst: Node).
            Line ::= new(a: Node, b: Node, c: Node, d: Node).
            Nocycle ::= new(node: Node).
            TwoEdge ::= new(first: Edge, second: Edge).

            Edge(x, z) :- Edge(x, y), Edge(y, z).
        }
        ".to_string();

    static ref ENV: Env<AtomicTerm> = AtomicTerm::load_program(GRAPH_PROGRAM.to_string());
    static ref GRAPH_DOMAIN: Domain<AtomicTerm> = ENV.get_domain_by_name("Graph").unwrap().clone();
    static ref GRAPH_TYPE_MAP: HashMap<String, AtomicType> = GRAPH_DOMAIN.meta_info().type_map().clone();
    static ref NODE_TYPE: AtomicType = GRAPH_TYPE_MAP.get("Node").unwrap().clone();
}

fn hops_dataflow<G, N>(edges: &Collection<G, (N, N)>, hops: i32) -> Collection<G, (N, N)>
where
    G: Scope,
    G::Timestamp: Lattice+Ord,
    N: ExchangeData+Hashable,
{
    let edges_arranged_by_key = edges.arrange_by_key();
    // Find all starting points and see how far they reach within n hops.
    let mut reachable = edges.map(|(q, _)| (q.clone(), q));

    for hop in 0 .. hops {
        reachable = reachable
            .map(|(q, y)| (y, q)) // (q, y) joins (y, z)
            .join_core(&edges_arranged_by_key, |_y, q, z| Some((q.clone(), z.clone())))
            //.inspect(move |x| println!("Round {:?} reachable: {:?}", hop, x))
            ;
    }
    
    reachable
        //.inspect(|x| println!("Final result {:?}", x))
}

fn transitive_closure_dataflow<G, N>(edges: &Collection<G, (N, N)>) -> Collection<G, (N, N)>
where
    G: Scope,
    G::Timestamp: Lattice+Ord,
    N: ExchangeData+Hashable,
{
    edges.iterate(|inner| {
        let edges = edges.enter(&inner.scope());
        // (x, y), (y, z) -> (x, z). (y, x) joins (y, z) = (y, (x, z))
        inner.map(|(x, y)| (y, x))
            .join(&inner)
            .map(|(_y, (x, z))| (x, z))
            .concat(&edges)
            //.distinct()
    })
}

#[derive(Hash, Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct WrapInt {
    val: usize
}

// Some lambda functions to convert `usize` to other types.
// fn convert_to_int(edges: Vec<(usize, usize)>) -> Vec<(AtomicPtrWrapper<usize>, AtomicPtrWrapper<usize>)> {
fn convert_to_int(edges: Vec<(usize, usize)>) -> Vec<(AtomicPtrWrapper<WrapInt>, AtomicPtrWrapper<WrapInt>)> {
    let mut int_map = HashMap::new();
    let mut ptr_edges = vec![];
    for (x, y) in edges {
        if !int_map.contains_key(&x) {
            let ptrx: AtomicPtrWrapper<WrapInt> = WrapInt { val: x }.into();
            int_map.insert(x, ptrx);
        }

        if !int_map.contains_key(&y) {
            let ptry: AtomicPtrWrapper<WrapInt> = WrapInt { val: y }.into();
            int_map.insert(y, ptry);
        }

        let ptrx = int_map.get(&x).unwrap();
        let ptry = int_map.get(&y).unwrap();

        if x == y {
            assert_eq!(ptrx, ptry);
        }

        ptr_edges.push( (ptrx.clone(), ptry.clone()) );
    }

    ptr_edges
}

// Integer to BigInt.
fn convert_to_bigint(edges: Vec<(usize, usize)>) -> Vec<(BigInt, BigInt)>{ 
    let mut bigint_edges = vec![];
    for (x, y) in edges {
        bigint_edges.push( (BigInt::from(x), BigInt::from(y)) );
    }
    bigint_edges
}

// Need to move a type map into the closure.
fn convert_to_term(edges: Vec<(usize, usize)>) -> Vec<(AtomicTerm, AtomicTerm)> {
    let mut term_edges = vec![];
    for (x, y) in edges {
        let term_astx = term(&format!("Node({}){}", x, "~")[..]).unwrap().1;
        let term_asty = term(&format!("Node({}){}", y, "~")[..]).unwrap().1;
        let termx = AtomicTerm::from_term_ast(&term_astx, &GRAPH_TYPE_MAP);
        let termy = AtomicTerm::from_term_ast(&term_asty, &GRAPH_TYPE_MAP);
        let term_edge = (termx, termy);
        term_edges.push(term_edge);
    }
    term_edges
}

fn convert_to_int_hashmap(edges: Vec<(usize, usize)>) -> Vec<(BTreeMap<usize, usize>, BTreeMap<usize, usize>)> {
    let mut int_map_edges = vec![];
    for (x, y) in edges {
        let mut mapx = BTreeMap::new();
        mapx.insert(x, x);
        mapx.insert(x+1, x+1);
        let mut mapy = BTreeMap::new();
        mapy.insert(y, y);
        mapy.insert(y+1, y+1);
        let int_map_edge = (mapx, mapy);
        int_map_edges.push(int_map_edge);
    }
    int_map_edges
}


fn convert_to_ptr_term(edges: Vec<(usize, usize)>) -> Vec<(AtomicPtrTerm, AtomicPtrTerm)> {
    let mut ptr_edges = vec![];
    let mut term_store = AtomicPtrTermStore::new(HashSet::new(), HashMap::new());
    // Use term_store to make sure there is no duplicates that has different allocations.
    for (x, y) in edges {
        let term_astx = term(&format!("Node({}){}", x, "~")[..]).unwrap().1;
        let termx = AtomicTerm::from_term_ast(&term_astx, &GRAPH_TYPE_MAP);
        let ptr_termx = term_store.intern(termx).clone();

        let term_asty = term(&format!("Node({}){}", y, "~")[..]).unwrap().1;
        let termy = AtomicTerm::from_term_ast(&term_asty, &GRAPH_TYPE_MAP);
        let ptr_termy = term_store.intern(termy).clone();
        let ptr_edge = (ptr_termx, ptr_termy);
        ptr_edges.push(ptr_edge);
    }
    ptr_edges
}

fn convert_to_term_hashmap(edges: Vec<(usize, usize)>) -> Vec<(BTreeMap<AtomicTerm, AtomicTerm>, BTreeMap<AtomicTerm, AtomicTerm>)> {
    let a: AtomicTerm = "a".into();
    let b: AtomicTerm = "b".into();
    let mut term_map_edges = vec![];

    for (x, y) in edges {
        let atom_enumx = AtomEnum::Int(BigInt::from(x));
        let atom_termx = AtomicTerm::create_atom_term(None, atom_enumx); 
        let nodex: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(NODE_TYPE.clone(), vec![atom_termx.into()], None)
        );

        let atom_enumy = AtomEnum::Int(BigInt::from(y));
        let atom_termy = AtomicTerm::create_atom_term(None, atom_enumy); 
        let nodey: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(NODE_TYPE.clone(), vec![atom_termy.into()], None)
        );

        let mut mapx = BTreeMap::new();
        mapx.insert(a.clone(), nodex.clone());
        mapx.insert(b.clone(), nodex.clone());

        let mut mapy = BTreeMap::new();
        mapy.insert(a.clone(), nodey.clone());
        mapy.insert(b.clone(), nodey.clone());

        let term_map_edge = (mapx, mapy);
        term_map_edges.push(term_map_edge);
    }

    term_map_edges
}

fn convert_to_formula_atomic_match(edges: Vec<(usize, usize)>) 
-> Vec<(AtomicPtrWrapper<Match<AtomicPtrTerm>>, AtomicPtrWrapper<Match<AtomicPtrTerm>>)> {
    let mut term_store = AtomicPtrTermStore::new(HashSet::new(), HashMap::new());
    let mut match_store = AtomicPtrMatchStore::new();
    let mut ptr_edges = vec![];

    for (x, y) in edges {
        let a: AtomicTerm = "a".into();
        let b: AtomicTerm = "b".into();
        let ptr_a = term_store.intern(a).clone();
        let ptr_b = term_store.intern(b).clone();

        let atom_enumx = AtomEnum::Int(BigInt::from(x));
        let atom_termx = AtomicTerm::create_atom_term(None, atom_enumx); 
        let nodex: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(NODE_TYPE.clone(), vec![atom_termx.into()], None)
        );
        let ptr_nodex = term_store.intern(nodex).clone();

        let mut mapx = BTreeMap::new();
        mapx.insert(ptr_a.clone(), ptr_nodex.clone());
        mapx.insert(ptr_b.clone(), ptr_nodex.clone());
        // println!("PRINTOUT MAP {:?} with length {:?}", mapx, mapx.len());

        let atom_enumy = AtomEnum::Int(BigInt::from(y));
        let atom_termy = AtomicTerm::create_atom_term(None, atom_enumy); 
        let nodey: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(NODE_TYPE.clone(), vec![atom_termy.into()], None)
        );
        let ptr_nodey = term_store.intern(nodey).clone();

        let mut mapy = BTreeMap::new();
        mapy.insert(ptr_a.clone(), ptr_nodey.clone());
        mapy.insert(ptr_b.clone(), ptr_nodey.clone());

        let matchx: Match<AtomicPtrTerm> = mapx.into();
        let matchy: Match<AtomicPtrTerm> = mapy.into();

        let ptr_matchx = match_store.intern(matchx);
        let ptr_matchy = match_store.intern(matchy);

        ptr_edges.push((ptr_matchx, ptr_matchy));
    }

    ptr_edges
}

#[test]
fn main() {
    println!("{:?}", std::env::args());

    // Default configuration with 2 hops and no inspection.
    let nodes_num: usize = std::env::args().nth(2).unwrap_or("100".to_string()).parse().unwrap_or(100);
    let edges_num: usize = std::env::args().nth(3).unwrap_or("20".to_string()).parse().unwrap_or(20);

    let seed: &[_] = &[1, 2, 3, 0];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
    let mut edge_set = HashSet::new();

    for _ in 0 .. edges_num {
        let num1 = rng1.gen_range(0, nodes_num);
        let num2 = rng1.gen_range(0, nodes_num);
        let edge = (num1, num2);
        edge_set.insert(edge);
    }

    let edges: Vec<(usize, usize)> = edge_set.into_iter().collect();

    for edge in edges.iter() {
        println!("Initial input: {:?}", edge);
    }

    //transitive();
    //transitive_formula();

    // RUST_TEST_NOCAPTURE=1 cargo test main -- 500 3000 2 false
    // hops();
    // hops_formula();
    // hops_formula_hashmap();
    // hops_differential_formula();

    // Formula Match is a hash map but check equality on pointers.

    println!("/***************** Integer *****************/");
    hops_computation(convert_to_int(edges.clone()));
    hops_computation(convert_to_bigint(edges.clone()));

    println!("/***************** Formula Term *****************/");
    hops_computation(convert_to_term(edges.clone()));
    hops_computation(convert_to_ptr_term(edges.clone()));

    println!("/***************** Hash Map *****************/");
    hops_computation(convert_to_int_hashmap(edges.clone()));
    hops_computation(convert_to_term_hashmap(edges.clone()));
    hops_computation(convert_to_formula_atomic_match(edges.clone()));
    
    // hops_differential_formula();
}

fn hops_computation<N>(edge_tuples: Vec<(N, N)>) where N: ExchangeData + Hashable + Hash {
    timely::execute_from_args(std::env::args(), move |worker| {
        let hops: i32 = std::env::args().nth(4).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(5).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let index = worker.index();
        // let peers = worker.peers();

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let output = hops_dataflow(&edges, hops);

            if inspect { output.inspect(|x| println!("The output: {:?}", x)); }

            let probe = output.probe();

            (input, probe)
        });

        for edge in edge_tuples.clone() { input.insert(edge); }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!(
                "Time: {:?} for computing hops with data structure {}", 
                timer.elapsed(), 
                type_name::<N>()
            );
        }

    }).unwrap();

}

fn hops_differential_formula() {
    let index = 0;
    let peers = 1;

    let nodes: usize = std::env::args().nth(2).unwrap_or("100".to_string()).parse().unwrap_or(100);
    let edges: usize = std::env::args().nth(3).unwrap_or("200".to_string()).parse().unwrap_or(200);
    // let hops: i32 = std::env::args().nth(4).unwrap_or("2".to_string()).parse().unwrap_or(2);
    let inspect: bool = std::env::args().nth(5).unwrap_or("false".to_string()).parse().unwrap_or(false);

    let program2 = "
        domain Graph {
            Node ::= new(name: Integer).
            Edge ::= new(src: Node, dst: Node).
            Path ::= new(src: Node, dst: Node).
            Line ::= new(a: Node, b: Node, c: Node, d: Node).
            Nocycle ::= new(node: Node).
            TwoEdge ::= new(first: Edge, second: Edge).

            Path(x, z) :- Edge(x, y), Edge(y, z).
        }
        ".to_string();

    let env2 = AtomicTerm::load_program(program2);
    let mut engine = DDEngine::new(env2);

    if inspect { engine.inspect = true; } else { engine.inspect = false; }

    // Parse string and install program in the engine.
    let m1 = engine.create_empty_model("m1", "Graph").clone();

    let seed: &[_] = &[1, 2, 3, index];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

    let mut session = Session::new(m1, &engine);
    let mut terms = vec![];

    // Load up graph data. Round-robin among workers.
    for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
        let num1 = rng1.gen_range(0, nodes);
        let num2 = rng1.gen_range(0, nodes);
        let edge_str = format!("Edge(Node({}), Node({}))", num1, num2);
        let edge_term = session.create_term(&edge_str).unwrap();

        if inspect {
            println!("Initial term: {:?}", edge_term);
        }

        terms.insert(0, edge_term);
    }

    let timer = std::time::Instant::now();
    session.add_terms(terms);
    println!("differential-formula finished after {:?}", timer.elapsed());
}

fn transitive_computation<F, N>(convert: F) 
where 
    F: Fn(usize) -> N + Sync + Send + 'static,
    N: ExchangeData+Hashable,
{
    timely::execute_from_args(std::env::args(), move |worker| {
        let timer = ::std::time::Instant::now();

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("20".to_string()).parse().unwrap_or(20);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = transitive_closure_dataflow(&edges).probe();
            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed); // rng for edge additions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            let edge = (convert(num1), convert(num2));
            input.update(edge, 1);
        }

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute transitive closure in {} finished after {:?}", type_name::<N>(), timer.elapsed());
        }

    }).unwrap();
}


