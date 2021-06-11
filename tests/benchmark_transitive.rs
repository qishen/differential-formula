#![feature(core_intrinsics)]
#![type_length_limit="1120927"]
extern crate rand;

use std::intrinsics::type_name;
use std::marker::*;
use std::hash::Hash;
use std::collections::*;
use differential_datalog::record::IntoRecord;
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

// Integer to BigInt.
fn convert_to_bigint(edges: Vec<(usize, usize)>) -> Vec<(BigInt, BigInt)>{ 
    let mut bigint_edges = vec![];
    for (x, y) in edges {
        bigint_edges.push( (BigInt::from(x), BigInt::from(y)) );
    }
    bigint_edges
}

#[test]
fn test_transitive_closure() {
    println!("{:?}", std::env::args());

    // Default configuration with 2 hops and no inspection.
    let nodes_num: usize = std::env::args().nth(2).unwrap_or("200".to_string()).parse().unwrap_or(100);
    let edges_num: usize = std::env::args().nth(3).unwrap_or("300".to_string()).parse().unwrap_or(20);

    let seed: &[_] = &[1, 2, 3, 4];
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

    let convert = |x| x as u32;
    transitive_computation(convert);
    // RUST_TEST_NOCAPTURE=1 cargo test --package differential-formula --test benchmark_transitive 
    // -- 200 300 2 false
}

fn transitive_computation<F, N>(convert: F) 
where 
    F: Fn(usize) -> N + Sync + Send + 'static,
    N: ExchangeData+Hashable,
{
    timely::execute_from_args(std::env::args(), move |worker| {
        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(1).unwrap_or("200".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("300".to_string()).parse().unwrap_or(20);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = transitive_closure_dataflow(&edges).probe();
            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng1: StdRng = SeedableRng::from_seed(seed); // rng for edge additions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            let edge = (convert(num1), convert(num2));
            // input.update(edge, 1);
            input.insert(edge);
        }

        let timer = ::std::time::Instant::now();
        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("Compute transitive closure in {} finished after {:?}", type_name::<N>(), timer.elapsed());
        }
    }).unwrap();
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
            if inspect { 
                output.inspect(|x| println!("The output: {:?}", x)); 
            }
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
