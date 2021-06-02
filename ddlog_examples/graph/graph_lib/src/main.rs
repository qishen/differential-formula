use std::collections::HashSet;
use rand::{Rng, SeedableRng, StdRng};
use graph_ddlog::api::HDDlog; //The DDLog Database engine itself
use graph_ddlog::Relations;
use graph_ddlog::relid2name;

// import all types defined by the datalog program itself
use graph_ddlog::typedefs::*;

// Trait that must be implemented by an instance of a DDlog program. 
// Type that represents a set of changes to DDlog relations.
// Returned by `DDlog::transaction_commit_dump_changes()`.
use differential_datalog::{
    DDlog, 
    DDlogDynamic
}; 
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
        let number_threads = 1;
        let track_complet_snapshot = false;
        let (hddlog, _init_state) = HDDlog::run(number_threads, track_complet_snapshot)?;

        return Ok(Self{hddlog});
    }

    pub fn flush_updates(&mut self, updates: Vec<Update<DDValue>>) -> Result<DeltaMap<DDValue>, String> {
        self.hddlog.transaction_start()?;
        self.hddlog.apply_updates(&mut updates.into_iter())?;
        let delta = self.hddlog.transaction_commit_dump_changes()?;
        
        return Ok(delta); 
    }

    pub fn create_nodes(&mut self, users: Vec<u32>) -> Vec<Update<DDValue>> {
        let updates = users.into_iter().map(|num| {
            Update::Insert {
                relid: Relations::Node as RelId,
                v: Node { id: num }.into_ddvalue(),
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn create_edges(&mut self, users: Vec<(u32, u32)>) -> Vec<Update<DDValue>> {
        let updates = users.into_iter().map(|(src, dst)| {
            Update::Insert {
                relid: Relations::Edge as RelId,
                v: Edge { src: Node { id: src }, dst: Node { id: dst } }.into_ddvalue(),
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

fn main() {
    println!("Print out arguments: {:?}", std::env::args());

    let nodes_num: usize = std::env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap_or(100);
    let edges_num: usize = std::env::args().nth(2).unwrap_or("20".to_string()).parse().unwrap_or(20);
    let debug = std::env::args().nth(3).unwrap_or("nodebug".to_string()) == "debug";

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
    let mut edge_set = HashSet::new();

    let mut i = 0;
    while i < edges_num {
        let num1 = rng1.gen_range(0, nodes_num);
        let num2 = rng1.gen_range(0, nodes_num);
        let edge = (num1 as u32, num2 as u32);
        // In case duplicates are generated
        if !edge_set.contains(&edge) {
            edge_set.insert(edge);
            i += 1;
        }
    }

    let edges: Vec<(u32, u32)> = edge_set.into_iter().collect();
    if debug {
        for edge in edges.iter() { println!("Initial input: {:?}", edge); }
    }

    let timer = std::time::Instant::now();
    let mut bse = DDLogGraph::new().unwrap();

    let mut updates = vec![];
    let edge_updates = bse.create_edges(edges);
    updates.extend(edge_updates);

    // println!("Loading Time: {} seconds", timer.elapsed().as_secs());
    // timer = std::time::Instant::now();

    let mut delta = bse.flush_updates(updates).unwrap();

    // Print out the delta after the changes to relations.
    // DDLogGraph::dump_delta(&delta);

    if debug {
        let path_changes = delta.get_rel(Relations::Path as RelId);
        for (val, weight) in path_changes.clone() {
            println!("{} with weight {}", val, weight);
        }
    }

    println!("Graph has {} nodes and {} edges. Running Time: {} milliseconds", 
        nodes_num, edges_num, timer.elapsed().as_millis());
}