#![feature(core_intrinsics)]
#![type_length_limit="1120927"]
extern crate rand;

use std::cell::*;
use std::intrinsics::type_name;
use std::marker::*;
use std::sync::*;
use std::hash::Hash;
use std::collections::BTreeMap;
use rand::{Rng, SeedableRng, StdRng};
use num::*;

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
use differential_formula::util::map::*;
use differential_formula::parser::combinator::*;

use timely::dataflow::*;
use timely::dataflow::scopes::*;
use timely::worker::*;
use timely::communication::*;

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
    N: ExchangeData+Hashable,
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
            //.distinct()
    })
}

#[test]
fn main() {
    println!("{:?}", std::env::args());

    //transitive();
    //transitive_formula();

    // RUST_TEST_NOCAPTURE=1 cargo test main -- 500 3000 2 false
    // hops();
    // hops_formula();
    // hops_formula_hashmap();
    // hops_differential_formula();

    // graph_computation(convert_to_integer, tran_closure_dataflow);

    // graph_computation(
    //     convert_to_integer,
    //     Arc::new(|edges: Collection<Child<Worker<Allocator>, i32>, (usize, usize)>| transitive_closure_dataflow(&edges))
    // );

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

    let env = AtomicTerm::load_program(program);
    let mut engine = DDEngine::new(env);
    let m = engine.create_empty_model("m", "Graph");
    // let marc = Arc::new(RefCell::new(m));
    let safem = Arc::new(RwLock::new(m));
    let node_type = engine.env.get_domain_by_name("Graph").unwrap().meta_info()
                              .get_type_by_name(&"Node".to_string()).unwrap();
    let node_type2 = node_type.clone();
    let env = engine.env.clone();
    let env2 = env.clone();

    // Some lambda functions to convert `usize` to other types.
    let convert_to_int = |x: usize| { x };

    let convert_to_wrapped_int = |num: usize| {
        let v1: UnsignedWrapper<usize> = num.into();
        let v2: HashableWrapper<UnsignedWrapper<usize>> = v1.into();
        return OrdWrapper { item: v2 };
    };

    let convert_to_bigint = |x: usize| { BigInt::from(x) };

    let convert_to_wrapped_bigint = |num: usize| {
        let v = BigInt::from(num);
        let v1: HashableWrapper<BigInt> = v.into();
        return OrdWrapper { item: v1 };
    };

    let convert_to_term = move |x: usize| {
        let graph_domain = env.get_domain_by_name("Graph").unwrap();
        let term_ast = term(&format!("Node({}){}", x, "~")[..]).unwrap().1;
        let term = AtomicTerm::from_term_ast(&term_ast, graph_domain.meta_info().type_map());
        let node_with_hash: HashableWrapper<AtomicTerm> = term.clone().into();
        let wrapped_node = OrdWrapper { item: node_with_hash };
        return wrapped_node;
    };

    // let convert_to_indexed_term = move |x: usize| {
    //     let graph_domain = env2.get_domain_by_name("Graph").unwrap();
    //     let term_ast = term(&format!("Node({}){}", x, "~")[..]).unwrap().1;
    //     let term = AtomicTerm::from_term_ast(&term_ast, graph_domain.meta_info().type_map());
    //     let indexed_term = IndexedTerm::new(&term, safem.clone());
    //     return indexed_term;
    // };

    let convert_to_int_hashmap = |num: usize| {
        let mut map = BTreeMap::new();
        map.insert(num, num);
        //map.insert(num+1, num+1);
        return map;
    };

    let convert_to_int_quick_hashmap = |num: usize| {
        let mut map = BTreeMap::new();
        map.insert(num, num);
        //map.insert(num+1, num+1);
        let wrapped_map: QuickHashOrdMap<usize, usize> = map.into();
        return wrapped_map;
    };

    let convert_to_term_hashmap = move |num: usize| {
        let x: AtomicTerm = "x".into();
        let y: AtomicTerm = "y".into();
        let atom_enum = AtomEnum::Int(BigInt::from(num));
        let atom_term = AtomicTerm::create_atom_term(None, atom_enum); 
        let node: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(node_type.clone(), vec![atom_term], None)
        );

        let mut map = BTreeMap::new();
        map.insert(x.clone(), node.clone());
        //map.insert(y.clone(), Arc::new(node.clone()));
        return map;
    };

    let convert_to_term_quick_hashmap = move |num: usize| {
        let x: AtomicTerm = "x".into();
        let y: AtomicTerm = "y".into();
        let atom_enum = AtomEnum::Int(BigInt::from(num));
        let atom_term = AtomicTerm::create_atom_term(None, atom_enum); 
        let node: AtomicTerm = AtomicTerm::Composite(
            AtomicComposite::new(node_type2.clone(), vec![atom_term], None)
        );

        let mut map = BTreeMap::new();
        map.insert(x.clone(), node.clone());
        //map.insert(y.clone(), Arc::new(node.clone()));

        let wrapped_map: QuickHashOrdMap<AtomicTerm, AtomicTerm> = map.into();
        return wrapped_map;
    };

    hops_computation(convert_to_int);
    hops_computation(convert_to_wrapped_int);
    hops_computation(convert_to_bigint);
    hops_computation(convert_to_wrapped_bigint);
    hops_computation(convert_to_term);
    // hops_computation(convert_to_indexed_term);
    
    // hops_computation(convert_to_int_hashmap);
    // hops_computation(convert_to_int_quick_hashmap);
    // hops_computation(convert_to_term_hashmap);
    // hops_computation(convert_to_term_quick_hashmap);
    
    // hops_differential_formula();
}


fn hops_computation<F, N>(convert: F)
where 
    F: Fn(usize) -> N + Sync + Send + 'static,
    N: ExchangeData+Hashable,
{
    timely::execute_from_args(std::env::args(), move |worker| {
        let index = worker.index();
        let peers = worker.peers();

        // Default configuration with 2 hops and no inspection.
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

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            // Using BigInt is slower than primitive integer.
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            let edge = (convert(num1), convert(num2));
            input.update(edge, 1);
            if inspect {
                println!("Initial tuple {:?}", (num1, num2));
            }
        }

        let timer = ::std::time::Instant::now();

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute hops in {} finished after {:?}", type_name::<N>(), timer.elapsed());
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

    let env = AtomicTerm::load_program(program);
    let mut engine = DDEngine::new(env);

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


