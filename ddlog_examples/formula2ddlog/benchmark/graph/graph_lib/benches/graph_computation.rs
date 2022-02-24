use graph_lib::*;
use glassbench::*;

static NODES_NUM: usize = 100;
static EDGES_NUM: usize = 150;


fn bench_graph_computation(bench: &mut Bench) {
    bench.task("Graph", |task| {
        let graph = Graph::gen_random_graph(NODES_NUM, EDGES_NUM);
        let nodes = graph.nodes();
        let edges = graph.edges();
        task.iter(move || {
            let mut bse = DDLogGraph::new(false).unwrap();
            let mut updates = vec![];
            let node_updates = bse.gen_node_updates(nodes.clone(), UpdateKind::Insert);
            let edge_updates = bse.gen_edge_updates(edges.clone(), UpdateKind::Insert);
            updates.extend(node_updates);
            updates.extend(edge_updates);
            // let mut delta = bse.flush_updates(updates).unwrap();
            bse.flush_updates(updates).unwrap();
        })
    });
}

fn bench_graph_computation_bigedge(bench: &mut Bench) {
    bench.task("Graph", |task| {
        let graph = Graph::gen_random_graph(NODES_NUM, EDGES_NUM);
        let nodes = graph.nodes();
        let edges = graph.edges();
        task.iter(move || {
            let mut bse = DDLogGraph::new(false).unwrap();
            let mut updates = vec![];
            let node_updates = bse.gen_node_updates(nodes.clone(), UpdateKind::Insert);
            let edge_updates = bse.gen_bigedge_updates(edges.clone(), UpdateKind::Insert);
            updates.extend(node_updates);
            updates.extend(edge_updates);
            // let mut delta = bse.flush_updates(updates).unwrap();
            bse.flush_updates(updates).unwrap();
        })
    });
}

glassbench!(
    "Graph Computation",
    bench_graph_computation,
    bench_graph_computation_bigedge,
    // you can pass other task defining functions here
);