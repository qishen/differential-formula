use graph_lib::*;
use glassbench::*;

fn bench_graph_computation(bench: &mut Bench) {
    bench.task("Graph", |task| {
        let nodes_num = 200;
        let edges_num = 100;
        task.iter(move || {
            let mut bse = DDLogGraph::new(false).unwrap();
            let mut updates = vec![];
            let graph = Graph::gen_random_graph(nodes_num, edges_num);
            let nodes = graph.nodes();
            let edges = graph.edges();
            let node_updates = bse.gen_node_updates(nodes, UpdateKind::Insert);
            let edge_updates = bse.gen_edge_updates(edges, UpdateKind::Delete);
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
    // you can pass other task defining functions here
);