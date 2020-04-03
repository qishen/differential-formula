extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use std::marker::*;
use std::ops::Mul;
use std::sync::Arc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{Rng, SeedableRng, StdRng};
use num::*;
use im::OrdMap;

use differential_dataflow::*;
use differential_dataflow::Hashable;
use differential_dataflow::input::Input;
use differential_dataflow::operators::Iterate;
use differential_dataflow::operators::Join;
use differential_dataflow::operators::Threshold;
use differential_dataflow::operators::arrange::*;
use differential_dataflow::operators::join::*;
// use differential_dataflow::operators::Count;
use differential_dataflow::operators::count::CountTotal;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::difference::*;

use differential_formula::term::*;
use differential_formula::engine::*;

use timely::dataflow::*;
use timely::dataflow::scopes::*;
use timely::worker::*;
use timely::communication::*;


fn main() {
    println!("{:?}", std::env::args());

    transitive();
    transitive_formula();

    /*
    hops();
    hops_formula();
    hops_formula_hashmap();
    hops_differential_formula();
    */

    //let convert_to_integer = |x| x;
    //let tran_closure_dataflow = |edges| transitive_closure_dataflow(edges);

    /*graph_computation(
        |x| x,
        |edges| transitive_closure_dataflow(&edges)
    );*/
}

fn parse_program() {

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
    engine.inspect = false;
    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(program);
    engine.install(env);
    let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
    let edge = domain.get_type(&"Edge".to_string());
    let node = domain.get_type(&"Node".to_string());

}

fn hops_dataflow<G, N>(edges: &Collection<G, (N, N)>, hops: i32) -> Collection<G, (N, N)>
where
    G: Scope,
    G::Timestamp: Lattice+Ord,
    N: ExchangeData+Hash,
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

        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("20".to_string()).parse().unwrap_or(20);
        let hops: i32 = std::env::args().nth(3).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(4).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = hops_dataflow(&edges, hops).probe();
            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            input.update((num1, num2), 1);
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


fn hops_formula_hashmap() {
    let result = timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("200".to_string()).parse().unwrap_or(200);
        let hops: i32 = std::env::args().nth(3).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(4).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = hops_dataflow(&edges, hops).probe();
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
        let env = DDEngine::parse_string(program);
        engine.install(env);
        let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
        let edge = domain.get_type(&"Edge".to_string());
        let node = domain.get_type(&"Node".to_string());

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

        let _x: Term = Variable::new("x".to_string(), vec![]).into();
        let _y: Term = Variable::new("y".to_string(), vec![]).into();
        let _z: Term = Variable::new("z".to_string(), vec![]).into();

        let x = Arc::new(_x);
        let y = Arc::new(_y);
        let z = Arc::new(_z);

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            //println!("{:?}, {:?}", num1, num2);
            let atom1: Term = Atom::Int(BigInt::from(num1)).into();
            let atom2: Term = Atom::Int(BigInt::from(num2)).into();
            let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
            let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

            let mut map1 = OrdMap::new();
            map1.insert(x.clone(), Arc::new(node1.clone()));
            let mut map2 = OrdMap::new();
            map2.insert(x.clone(), Arc::new(node2.clone()));
            let edge_tuple = (map1, map2);

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





fn hops_formula() {
    let result = timely::execute_from_args(std::env::args(), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
        let edges: usize = std::env::args().nth(2).unwrap_or("200".to_string()).parse().unwrap_or(200);
        let hops: i32 = std::env::args().nth(3).unwrap_or("2".to_string()).parse().unwrap_or(2);
        let inspect: bool = std::env::args().nth(4).unwrap_or("false".to_string()).parse().unwrap_or(false);

        let (mut input, probe) = worker.dataflow(|scope| {
            let (input, edges) = scope.new_collection();
            let probe = hops_dataflow(&edges, hops).probe();
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
        let env = DDEngine::parse_string(program);
        engine.install(env);
        let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
        let edge = domain.get_type(&"Edge".to_string());
        let node = domain.get_type(&"Node".to_string());

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            //println!("{:?}, {:?}", num1, num2);
            let atom1: Term = Atom::Int(BigInt::from(num1)).into();
            let atom2: Term = Atom::Int(BigInt::from(num2)).into();
            let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
            let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

            let edge_tuple = (Arc::new(node1.clone()), Arc::new(node2.clone()));

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

fn hops_differential_formula() {
    let index = 0;
    let peers = 1;

    let nodes: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
    let edges: usize = std::env::args().nth(2).unwrap_or("200".to_string()).parse().unwrap_or(200);
    let hops: i32 = std::env::args().nth(3).unwrap_or("2".to_string()).parse().unwrap_or(2);
    let inspect: bool = std::env::args().nth(4).unwrap_or("false".to_string()).parse().unwrap_or(false);

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

    if inspect {
        engine.inspect = true;
    } else {
        engine.inspect = false;
    }

    // Parse string and install program in the engine.
    let env = DDEngine::parse_string(program);
    engine.install(env);
    let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
    let edge = domain.get_type(&"Edge".to_string());
    let node = domain.get_type(&"Node".to_string());

    let seed: &[_] = &[1, 2, 3, index];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
    //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

    let mut session = engine.create_session("Graph", Some("m"));
    let mut terms: Vec<Arc<Term>> = vec![];

    // Load up graph data. Round-robin among workers.
    for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
        let num1 = rng1.gen_range(0, nodes);
        let num2 = rng1.gen_range(0, nodes);
        //println!("{:?}, {:?}", num1, num2);
        let atom1: Term = Atom::Int(BigInt::from(num1)).into();
        let atom2: Term = Atom::Int(BigInt::from(num2)).into();
        let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
        let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

        let edge_term: Term = Composite::new(
            edge.clone(), 
            vec![Arc::new(node1), Arc::new(node2)], 
            None).into();

        if inspect {
            println!("Initial term: {:?}", edge_term);
        }

        terms.insert(0, Arc::new(edge_term));
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
        let env = DDEngine::parse_string(program);
        engine.install(env);
        let domain = engine.get_domain("Graph".to_string()).unwrap().clone();
        let edge = domain.get_type(&"Edge".to_string());
        let node = domain.get_type(&"Node".to_string());

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        //let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        let mut session = engine.create_session("Graph", Some("m"));
        let mut terms: Vec<Arc<Term>> = vec![];

        // Load up graph data. Round-robin among workers.
        for _ in 0 .. (edges / peers) + if index < (edges % peers) { 1 } else { 0 } {
            let num1 = rng1.gen_range(0, nodes);
            let num2 = rng1.gen_range(0, nodes);
            //println!("{:?}, {:?}", num1, num2);
            let atom1: Term = Atom::Int(BigInt::from(num1)).into();
            let atom2: Term = Atom::Int(BigInt::from(num2)).into();
            let node1: Term = Composite::new(node.clone(), vec![Arc::new(atom1)], None).into();
            let node2: Term = Composite::new(node.clone(), vec![Arc::new(atom2)], None).into();

            let edge_tuple = (Arc::new(node1.clone()), Arc::new(node2.clone()));
            //let edge_tuple = (node1.clone(), node2.clone());
            input.insert(edge_tuple);

            let edge_term: Term = Composite::new(
                edge.clone(), 
                vec![Arc::new(node1), Arc::new(node2)], 
                None).into();

            terms.insert(0, Arc::new(edge_term));
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


/*fn test_term() {

    let atom: Term = Atom::Int(BigInt::from(888)).into();
    let atom1: Term = Atom::Int(BigInt::from(666)).into();
    let node: Term = Composite::new(Node.clone(), vec![Arc::new(atom.clone())], None).into();
    let node2: Term = Composite::new(Node.clone(), vec![Arc::new(atom.clone())], None).into();
    let node3: Term = Composite::new(Node.clone(), vec![Arc::new(atom1.clone())], None).into();
    
    
    let edge0: Term = Composite::new(Edge.clone(), vec![Arc::new(node.clone()), Arc::new(node2.clone())], None).into();
    let edge1: Term = Composite::new(Edge.clone(), vec![Arc::new(node.clone()), Arc::new(node3.clone())], None).into();
    let edge2: Term = edge1.clone();

    let node_arc = Arc::new(node.clone());
    //let node2 = node.clone();
    
    let number: i32 = 888;
    let number2 = number.clone();

    let mut hasher = DefaultHasher::new();
    let timer = std::time::Instant::now();
    //number.hash(&mut hasher);
    //let hvalue = hasher.finish();
    println!("equal: {:?}", number > number2);
    
    //let number_cloned = number.clone();

    println!("Time for computing hash of primitive: {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    //let hvalue = node.hashed();
    //println!("equal: {:?}", Arc::new(edge0) == Arc::new(edge1));
    println!("equal: {:?}", edge0 > edge1);
    //node.clone();
    //node_arc.clone();
    //atom.clone();
    //BigInt::from(888).clone();
    println!("Time for computing hash of formula term: {:?}", timer.elapsed());
}
*/