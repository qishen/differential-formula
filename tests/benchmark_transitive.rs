#![type_length_limit="1120927"]
extern crate rand;

use std::marker::*;
use std::sync::Arc;
use std::hash::Hash;
use std::collections::BTreeMap;
use rand::{Rng, SeedableRng, StdRng};
use num::*;

use differential_dataflow::*;
use differential_dataflow::input::Input;
use differential_dataflow::operators::Iterate;
use differential_dataflow::operators::Join;
use differential_dataflow::operators::Threshold;
use differential_dataflow::operators::arrange::*;
use differential_dataflow::operators::join::*;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::hashable::*;

use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::util::*;

use timely::dataflow::*;
use timely::dataflow::scopes::*;
use timely::worker::*;
use timely::communication::*;

#[test]
fn main() {
    println!("{:?}", std::env::args());

    //transitive();
    //transitive_formula();

    // RUST_TEST_NOCAPTURE=1 cargo test main -- 500 3000 2 false
    hops();
    hops_formula();
    hops_formula_hashmap();
    hops_differential_formula();

    //let convert_to_integer = |x| x;
    //let tran_closure_dataflow = |edges| transitive_closure_dataflow(edges);

    /*graph_computation(
        |x| x,
        |edges| transitive_closure_dataflow(&edges)
    );*/
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

    for _hop in 0 .. hops {
        reachable = reachable
            .map(|(q, y)| (y, q)) // (q, y) joins (y, z)
            .join_core(&edges_arranged_by_key, |_y, q, z| Some((q.clone(), z.clone())));
    }
    
    reachable
        //.inspect(|x| println!("Final result {:?}", x))
}

fn transitive_closure_dataflow<G, N>(edges: &Collection<G, (N, N)>) -> Collection<G, (N, N)>
where
    G: Scope,
    G::Timestamp: Lattice+Ord,
    N: ExchangeData+Hash,
    //R: ExchangeData+Abelian,
    //R: Mul<R, Output=R>,
    //R: From<i8>,
{
    edges.iterate(|inner| {
        let edges = edges.enter(&inner.scope());
        // (x, y), (y, z) -> (x, z). (y, x) joins (y, z) = (y, (x, z))
        inner.map(|(x, y)| (y, x))
            .join(&inner)
            .map(|(_y, (x, z))| (x, z))
            .concat(&edges)
            .distinct()
    })
}


fn graph_computation<G, N, F1, F2>(convert: F1, dataflow: F2) 
where
    F1: Fn(usize) -> N + Send + Sync + 'static,
    F2: Fn(&Collection<Child<Worker<Allocator>, i32>, (N, N)>) -> Collection<G, (N, N)> + Send + Sync + 'static,
    G: Scope<Timestamp=i32>,
    G::Timestamp: Lattice+Ord,
    N: ExchangeData+Hash,
{
    timely::execute_from_args(std::env::args(), move |worker| {
        let timer = ::std::time::Instant::now();

        let index = worker.index();
        let peers = worker.peers();

        // Set default numbers for nodes and edges if not specified in cmd arguments.
        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("20".to_string()).parse().unwrap_or(20);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = dataflow(&edges).probe();
            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            let node1 = convert(num1);
            let node2 = convert(num2);
            input.update((node1, node2), 1)
        }

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Graph computation finished after {:?}", timer.elapsed());
        }

    }).unwrap();
}

fn hops() {
    timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(2).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(3).unwrap_or("20".to_string()).parse().unwrap_or(20);
        let hops: i32 = std::env::args().nth(4).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(5).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let output = hops_dataflow(&edges, hops);
            if inspect {
                output.inspect(|x| println!("The output: {:?}", x));
            }
            let probe = output.probe();
            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            // Using BigInt is slower than primitive integer.
            let num1 = BigInt::from(rng1.gen_range(0, nodes));
            let num2 = BigInt::from(rng1.gen_range(0, nodes));
            input.update((num1.clone(), num2.clone()), 1);
            if inspect {
                println!("Initial tuple {:?}", (num1, num2));
            }
        }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute hops in integers finished after {:?}", timer.elapsed());
        }

    }).unwrap();

}

// Use a tuple of two Formula terms as data structure but still run on native differential dataflow.
fn hops_formula() {
    timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(2).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(3).unwrap_or("200".to_string()).parse().unwrap_or(200);
        let hops: i32 = std::env::args().nth(4).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(5).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let output = hops_dataflow(&edges, hops);
            if inspect {
                output.inspect(|x| println!("The output: {:?}", x));
            }
            let probe = output.probe();
            (input, probe)
        });

        let program = "
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

        let mut engine = DDEngine::new();

        // Parse string and install program in the engine.
        engine.install(program);
        let m = engine.create_empty_model("m", "Graph");
        let session = Session::new(m, &engine);

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);

            let node1 = session.create_term(&format!("Node({})", num1)).unwrap();
            let node2 = session.create_term(&format!("Node({})", num2)).unwrap();

            let node1_with_hash: HashableWrapper<Term> = node1.clone().into();
            let node2_with_hash: HashableWrapper<Term> = node2.clone().into();
            // println!("{:?}", node1_with_hash);
            let wrapped_node1 = OrdWrapper { item: node1_with_hash };
            let wrapped_node2 = OrdWrapper { item: node2_with_hash };

            // let edge_tuple = (node1, node2);
            let edge_tuple = (wrapped_node1, wrapped_node2);

            if inspect {
                println!("Initial term tuple {:?}", edge_tuple);
            }

            input.insert(edge_tuple);
        }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute hops in terms finished after {:?}", timer.elapsed());
        }

    }).unwrap();
}

// Use native differential formula but each node is a hashmap with formula terms.
fn hops_formula_hashmap() {
    timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(2).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(3).unwrap_or("200".to_string()).parse().unwrap_or(200);
        let hops: i32 = std::env::args().nth(4).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(5).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let output = hops_dataflow(&edges, hops);
            if inspect {
                output.inspect(|x| println!("The output: {:?}", x));
            }
            let probe = output.probe();
            (input, probe)
        });

        let program = "
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

        let mut engine = DDEngine::new();

        if inspect {
            engine.inspect = true;
        } else {
            engine.inspect = false;
        }

        engine.install(program);

        let domain = engine.env.get_domain_by_name("Graph").unwrap().clone();
        let node = domain.get_type(&"Node".to_string());

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

        let _x: Term = Variable::new("x".to_string(), vec![]).into();
        let _y: Term = Variable::new("y".to_string(), vec![]).into();
        let x = Arc::new(_x);
        let y = Arc::new(_y);

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            let atom1: Term = AtomEnum::Int(BigInt::from(num1)).into();
            let atom2: Term = AtomEnum::Int(BigInt::from(num2)).into();
            let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
            let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

            // Each map has an unique form that is used to compare ordering when two maps have the same
            // hash then just check if they have same unique form.
            let mut map1 = BTreeMap::new();
            map1.insert(x.clone(), Arc::new(node1.clone()));
            map1.insert(y.clone(), Arc::new(node1.clone()));
            //println!("Hash of {:?} is {}", map1, map1.hashed());
            let wrapped_map1: QuickHashOrdMap<Arc<Term>, Arc<Term>> = map1.into();

            let mut map2 = BTreeMap::new();
            map2.insert(x.clone(), Arc::new(node2.clone()));
            map2.insert(y.clone(), Arc::new(node2.clone()));
            //println!("Hash of {:?} is {}", map2, map2.hashed());
            let wrapped_map2: QuickHashOrdMap<Arc<Term>, Arc<Term>> = map2.into();

            // let map1_with_hash: HashableWrapper<OrdMap<Term, Term>> = map1.clone().into();
            // let map2_with_hash: HashableWrapper<OrdMap<Term, Term>> = map2.clone().into();
            // println!("{:?}", node1_with_hash);
            // let wrapped_map1 = OrdWrapper { item: map1_with_hash };
            // let wrapped_map2 = OrdWrapper { item: map2_with_hash };

            let edge_tuple = (wrapped_map1, wrapped_map2);
            // let edge_tuple = (wrapped_map1, wrapped_map2);

            if inspect {
                println!("Initial term tuple {:?}", edge_tuple);
            }

            input.insert(edge_tuple);
        }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute hops in hashmaps finished after {:?}", timer.elapsed());
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

    let program = "
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

    let mut engine = DDEngine::new();

    if inspect { engine.inspect = true; } 
    else { engine.inspect = false; }

    // Parse string and install program in the engine.
    engine.install(program);
    let m1 = engine.create_empty_model("m1", "Graph").clone();

    let seed: &[_] = &[1, 2, 3, index];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

    let mut session = Session::new(m1, &engine);
    let mut terms: Vec<Term> = vec![];

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

fn transitive() {
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
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            input.update((num1, num2), 1)
        }

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute transitive closure in integers finished after {:?}", timer.elapsed());
        }

    }).unwrap();
}


fn transitive_formula() {

    let result = timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("200".to_string()).parse().unwrap_or(200);
        let inspect: bool = std::env::args().nth(3).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = transitive_closure_dataflow(&edges).probe();
            (input, probe)
        });

        let program = "
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

        let mut engine = DDEngine::new();

        if inspect {
            engine.inspect = true;
        } else {
            engine.inspect = false;
        }

        // Parse string and install program in the engine.
        engine.install(program);
        let domain = engine.env.get_domain_by_name("Graph").unwrap().clone();
        let edge = domain.get_type(&"Edge".to_string());
        let node = domain.get_type(&"Node".to_string());

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        let empty_model = engine.create_empty_model("m", "Graph");
        let mut session = Session::new(empty_model, &engine);
        let mut terms: Vec<Term> = vec![];

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            //println!("{:?}, {:?}", num1, num2);
            let atom1: Term = AtomEnum::Int(BigInt::from(num1)).into();
            let atom2: Term = AtomEnum::Int(BigInt::from(num2)).into();
            let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
            let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

            let edge_tuple = (Arc::new(node1.clone()), Arc::new(node2.clone()));
            //let edge_tuple = (node1.clone(), node2.clone());
            input.insert(edge_tuple);

            let edge_term: Term = Composite::new(
                edge.clone(), 
                vec![Arc::new(node1), Arc::new(node2)], 
                None).into();

            terms.insert(0, edge_term);
        }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute transitive closure in terms finished after {:?}", timer.elapsed());
        }

        let timer = std::time::Instant::now();
        session.add_terms(terms);

        println!("differential-formula finished after {:?}", timer.elapsed());

    }).unwrap();
} 