use std::collections::HashSet;
use rand::{Rng, SeedableRng, StdRng};

use graph_ddlog::Relations;
use graph_ddlog::relid2name;
// import all types defined by the datalog program itself
use graph_ddlog::typedefs::*;

// Trait that must be implemented by an instance of a DDlog program. 
// Type that represents a set of changes to DDlog relations.
// Returned by `DDlog::transaction_commit_dump_changes()`.
use differential_datalog::{DDlog, DDlogDynamic}; 
use differential_datalog::program::config::{Config, ProfilingConfig};
use differential_datalog::api::HDDlog;
use differential_datalog::DeltaMap; // A trait representing the changes resulting from a given update.
use differential_datalog::ddval::DDValue; // A generic DLog value type
use differential_datalog::ddval::DDValConvert; //Another helper trair
use differential_datalog::program::RelId; // Numeric relations id
use differential_datalog::program::Update; // A type representing updates to the database
// use differential_datalog::record::{FromRecord, IntoRecord}; // A type representing individual facts

pub struct DDLogGraph{
    hddlog: HDDlog,
}

impl DDLogGraph {
    pub fn new()  -> Result<DDLogGraph, String> {
        let config = Config::new()
            .with_timely_workers(1)
            .with_profiling_config(ProfilingConfig::SelfProfiling);
        // let (hddlog, init_state) = formula2ddlog_ddlog::run_with_config(config, false); 
        let (hddlog, init_state) = graph_ddlog::run_with_config(config, false)?;
        Self::dump_delta(&init_state);
        return Ok(Self{hddlog});
    }

    pub fn flush_updates(&mut self, updates: Vec<Update<DDValue>>) -> Result<DeltaMap<DDValue>, String> {
        self.hddlog.transaction_start()?;
        self.hddlog.apply_updates(&mut updates.into_iter())?;
        let delta = self.hddlog.transaction_commit_dump_changes()?;
        return Ok(delta); 
    }

    pub fn create_nodes(&mut self, users: Vec<u64>) -> Vec<Update<DDValue>> {
        let updates = users.into_iter().map(|num| {
            Update::Insert {
                relid: Relations::NodeInput as RelId,
                v: Node { name: Graph_AUTOTYPE0::Graph_AUTOTYPE0_usize{usize_field: num} }.into_ddvalue(),
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn create_edges(&mut self, users: Vec<(u64, u64)>) -> Vec<Update<DDValue>> {
        let updates = users.into_iter().map(|(src, dst)| {
            Update::Insert {
                relid: Relations::EdgeInput as RelId,
                v: Edge { 
                    src: Node { name: Graph_AUTOTYPE0::Graph_AUTOTYPE0_usize{usize_field: src} }, 
                    dst: Node { name: Graph_AUTOTYPE0::Graph_AUTOTYPE0_usize{usize_field: dst} } 
                }.into_ddvalue(),
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn dump_delta(delta: &DeltaMap<DDValue>) {
        for (rel, changes) in delta.iter() {
            println!("Changes to relation {}", relid2name(*rel).unwrap());
            for (val, weight) in changes.iter() {
                println!("{} {:+}", val, weight);
            }
        }
    }

    pub fn stop(&mut self){
        self.hddlog.stop().unwrap();
    }
}


fn generate_graph(nodes_num: usize, edges_num: usize, seed: &[usize]) -> (Vec<u64>, Vec<(u64, u64)>) {
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
    let mut node_set = HashSet::new();
    let mut edge_set = HashSet::new();
    let mut i = 0;
    while i < edges_num {
        let num1 = rng1.gen_range(0, nodes_num);
        let num2 = rng1.gen_range(0, nodes_num);
        node_set.insert(num1 as u64);
        node_set.insert(num2 as u64);
        let edge = (num1 as u64, num2 as u64);
        // In case duplicates are generated
        if !edge_set.contains(&edge) {
            edge_set.insert(edge);
            i += 1;
        }
    }
    let nodes: Vec<u64> = node_set.into_iter().collect();
    let edges: Vec<(u64, u64)> = edge_set.into_iter().collect();
    (nodes, edges)
}


fn main() {
    println!("Print out arguments: {:?}", std::env::args());

    let nodes_num: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
    let edges_num: usize = std::env::args().nth(2).unwrap_or("20".to_string()).parse().unwrap_or(20);
    let target: String = std::env::args().nth(3).unwrap_or("ddlog".to_string());
    let debug = std::env::args().nth(4).unwrap_or("nodebug".to_string()) == "debug";

    let seed: &[_] = &[1, 2, 3, 4];
    let (nodes, edges) = generate_graph(nodes_num, edges_num, seed);

    if target == "formula" {
        for (src, dst) in edges.iter() { println!("Edge(Node({}), Node({})).", src, dst); }
    } else {
        if debug {
            for edge in edges.iter() { println!("Initial input: {:?}", edge); }
        }

        let mut timer = std::time::Instant::now();
        let mut bse = DDLogGraph::new().unwrap();

        let mut updates = vec![];
        let node_updates = bse.create_nodes(nodes);
        let edge_updates = bse.create_edges(edges);
        updates.extend(node_updates);
        updates.extend(edge_updates);

        // println!("Loading Time: {} seconds", timer.elapsed().as_secs());
        // timer = std::time::Instant::now();

        let mut delta = bse.flush_updates(updates).unwrap();
        if debug {
            DDLogGraph::dump_delta(&delta);
        }

        println!("Graph has {} nodes and {} edges. Running Time: {} milliseconds", 
            nodes_num, edges_num, timer.elapsed().as_millis());

        let new_edges_num = edges_num / 10;
        let (new_nodes, new_edges) = generate_graph(nodes_num, new_edges_num / 10, &[4,3,2,1]);
        let mut new_updates = vec![]; 
        let new_node_updates = bse.create_nodes(new_nodes);
        let new_edge_updates = bse.create_edges(new_edges);
        new_updates.extend(new_node_updates);
        new_updates.extend(new_edge_updates);

        timer = std::time::Instant::now();
        delta = bse.flush_updates(new_updates).unwrap();
        if debug {
            DDLogGraph::dump_delta(&delta);
        }

        println!("Add {} new edges. Running Time: {} milliseconds", new_edges_num, timer.elapsed().as_millis());
    }
}