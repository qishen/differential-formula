use std::collections::BTreeSet;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use graph_ddlog::Relations;
use graph_ddlog::relid2name;
use graph_ddlog::typedefs::*;

use differential_datalog::{DDlog, DDlogDynamic}; 
use differential_datalog::program::config::{Config, ProfilingConfig};
use differential_datalog::api::HDDlog;
use differential_datalog::DeltaMap; // A trait representing the changes resulting from a given update.
use differential_datalog::ddval::{DDVal, DDValConvert, DDValMethods, DDValue}; // A generic DLog value type
// use differential_datalog::ddval::DDValConvert; //Another helper trair
use differential_datalog::program::RelId; // Numeric relations id
use differential_datalog::program::Update; // A type representing updates to the database

pub struct Graph {
    // The elements must be ordered in nodes and edges
    node_set: BTreeSet<usize>,
    edge_set: BTreeSet<(usize, usize)>,
    seed: u64,
}

impl Graph {
    pub fn new_empty() -> Self {
        Self { 
            node_set: BTreeSet::new(), 
            edge_set: BTreeSet::new(),
            seed: 0
        }
    }

    pub fn nodes(&self) -> Vec<usize> {
        self.node_set.clone().into_iter().collect()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.edge_set.clone().into_iter().collect()
    }

    pub fn add_node(&mut self, node: usize) -> bool {
        if self.node_set.contains(&node) {
            return false;
        } else {
            self.node_set.insert(node);
            return true;
        }
    }

    pub fn add_edge(&mut self, edge: (usize, usize)) -> bool {
        if self.edge_set.contains(&edge) {
            return false;
        } else {
            let (src, dst) = edge;
            if !self.node_set.contains(&src) { self.add_node(src); }
            if !self.node_set.contains(&dst) { self.add_node(dst); }
            self.edge_set.insert(edge);
            return true;
        }
    }

    pub fn add_random_edge(&mut self) -> (usize, usize) {
        let mut rng = StdRng::seed_from_u64(self.seed); 
        // Increment the seed so it won't generate the same graph in the next round
        self.seed += 1;
        let num1 = rng.gen_range(0..self.node_set.len());
        let num2 = rng.gen_range(0..self.node_set.len());
        let mut edge = (num1, num2);
        while self.contains_edge(edge) {
            let num1 = rng.gen_range(0..self.node_set.len());
            let num2 = rng.gen_range(0..self.node_set.len());
            edge = (num1, num2);
        };
        edge
    }

    pub fn contains_node(&self, node: usize) -> bool {
        self.node_set.contains(&node)
    }

    pub fn contains_edge(&self, edge: (usize, usize)) -> bool {
        self.edge_set.contains(&edge)
    }

    pub fn gen_random_graph(nodes_num: usize, edges_num: usize) -> Graph {
        let mut graph = Graph::new_empty();
        for i in 0 .. nodes_num {
            graph.add_node(i);
        }

        let mut rng1: StdRng = SeedableRng::seed_from_u64(graph.seed); 
        // Increment the seed so it won't generate the same graph in the next round
        graph.seed += 1;
        let mut count = 0;
        while count < edges_num {
            let num1 = rng1.gen_range(0..nodes_num);
            let num2 = rng1.gen_range(0..nodes_num);
            let edge = (num1, num2);
            // Check if a duplicate edge is generated, if so disgard and generate a new edge 
            if !graph.edge_set.contains(&edge) {
                graph.add_edge(edge);
                count += 1;
            }
        }
        graph
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UpdateKind {
    Insert,
    Delete,
    DeleteByKey,
    // Modify
}

pub struct DDLogGraph{
    debug: bool,
    hddlog: HDDlog,
}

impl DDLogGraph {
    pub fn new(debug: bool)  -> Result<DDLogGraph, String> {
        let config = Config::new()
            .with_timely_workers(1)
            .with_profiling_config(ProfilingConfig::None);
        // let (hddlog, init_state) = formula2ddlog_ddlog::run_with_config(config, false); 
        let (hddlog, init_state) = graph_ddlog::run_with_config(config, false)?;
        Self::dump_delta(&init_state);
        return Ok(Self { debug, hddlog });
    }

    pub fn flush_updates(&mut self, updates: Vec<Update<DDValue>>) -> Result<DeltaMap<DDValue>, String> {
        self.hddlog.transaction_start()?;
        self.hddlog.apply_updates(&mut updates.into_iter())?;
        let delta = self.hddlog.transaction_commit_dump_changes()?;
        return Ok(delta); 
    }

    pub fn gen_node_updates(&mut self, nodes: Vec<usize>, kind: UpdateKind) -> Vec<Update<DDValue>> {
        let updates = nodes.into_iter().map(|num| {
            let relid = Relations::NodeInput as RelId;
            let v = Node { name: num as u64 }.into_ddvalue();
            match kind {
                UpdateKind::Insert => Update::Insert { relid, v },
                UpdateKind::Delete => Update::DeleteValue { relid, v },
                UpdateKind::DeleteByKey => Update::DeleteKey { relid, k: v },
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn gen_edge_updates(&mut self, edges: Vec<(usize, usize)>, kind: UpdateKind) -> Vec<Update<DDValue>> {
        let updates = edges.into_iter().map(|(src, dst)| {
            let relid = Relations::EdgeInput as RelId;
            let v = Edge { 
                src: Node { name: src as u64 }, 
                dst: Node { name: dst as u64 } 
            }.into_ddvalue();
            match kind {
                UpdateKind::Insert => Update::Insert { relid, v },
                UpdateKind::Delete => Update::DeleteValue { relid, v },
                UpdateKind::DeleteByKey => Update::DeleteKey { relid, k: v },
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn gen_bigedge_updates(&mut self, edges: Vec<(usize, usize)>, kind: UpdateKind) -> Vec<Update<DDValue>> {
        let updates = edges.into_iter().map(|(src, dst)| {
            let relid = Relations::BigEdgeInput as RelId;
            let v = BigEdge { 
                src: ddlog_std::to_any(Node { name: src as u64 }), 
                dst: ddlog_std::to_any(Node { name: dst as u64 }) 
            }.into_ddvalue();
            match kind {
                UpdateKind::Insert => Update::Insert { relid, v },
                UpdateKind::Delete => Update::DeleteValue { relid, v },
                UpdateKind::DeleteByKey => Update::DeleteKey { relid, k: v },
            }
        }).collect::<Vec<_>>();

        updates
    } 

    pub fn init_graph(&mut self, nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> (usize, usize, u128) {
        let (nodes_num, edges_num, time_elapsed) = self.update_graph(nodes, edges, UpdateKind::Insert);
        (nodes_num, edges_num, time_elapsed)
    }

    pub fn update_graph(&mut self, nodes: Vec<usize>, edges: Vec<(usize, usize)>, kind: UpdateKind) -> (usize, usize, u128) {
        let nodes_num = nodes.len();
        let edges_num = edges.len();
        let mut updates = vec![];
        let node_updates = self.gen_node_updates(nodes, kind);
        let edge_updates = self.gen_edge_updates(edges, kind);
        updates.extend(node_updates);
        updates.extend(edge_updates);

        let timer = std::time::Instant::now();
        let delta = self.flush_updates(updates).unwrap();
        let time_elapsed = timer.elapsed().as_millis();

        if self.debug { 
            DDLogGraph::dump_delta(&delta); 
        }

        (nodes_num, edges_num, time_elapsed)
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