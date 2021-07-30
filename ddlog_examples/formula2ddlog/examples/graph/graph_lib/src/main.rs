use graph_lib::*;

// Usage:
// cargo run --release -- <NODE> <EDGE> <TARGET> debug
fn main() {
    // println!("Print out arguments: {:?}", std::env::args());
    let nodes_num: usize = std::env::args().nth(1).unwrap_or("0".to_string()).parse().unwrap_or(0);
    let edges_num: usize = std::env::args().nth(2).unwrap_or("0".to_string()).parse().unwrap_or(0);
    let target: String = std::env::args().nth(3).unwrap_or("ddlog".to_string());
    // By default, do not print out delta.
    let debug = std::env::args().nth(4).unwrap_or("nodebug".to_string()) == "debug";

    let graph = Graph::gen_random_graph(nodes_num, edges_num);
    let nodes = graph.nodes();
    let edges = graph.edges();

    // Print out the header
    println!("nodes,edges,init,add,del");

    if target == "formula" {
        for (src, dst) in edges.iter() { 
            println!("Edge(Node({}), Node({})).", src, dst); 
        }
    } else if target == "formula_bench" {
        for i in 0 .. 30 {
            let nodes_num = 2000;
            let edges_num = 100 * (i + 1);
            let graph = Graph::gen_random_graph(nodes_num, edges_num);
            let edges = graph.edges();
            for (src, dst) in edges.iter() { 
                println!("Edge(Node({}), Node({})).", src, dst); 
            }
        }
    } else if target == "ddlog" {
        let mut bse = DDLogGraph::new(debug).unwrap();
        let (_,_,t) = bse.init_graph(nodes, edges);
        println!("Nodes: {}, Edges: {}, Running time: {} milli seconds", nodes_num, edges_num, t);
    } else if target == "ddlog_bench" {
        for i in 0 .. 30 {
            let nodes_num = 2000;
            let edges_num = 100 * (i + 1);
            let mut graph = Graph::gen_random_graph(nodes_num, edges_num);
            let nodes = graph.nodes();
            let edges = graph.edges();
            let mut bse = DDLogGraph::new(debug).unwrap();
            let (_,_, t1) = bse.init_graph(nodes, edges);
            
            // Create more random edges to test incremental update 
            let mut new_edges = vec![];
            for _ in 0 .. 5 {
                let new_edge = graph.add_random_edge();
                new_edges.push(new_edge);
            }

            // Add new edges that have not been added to current graph before
            let (_,_,t2) = bse.update_graph(vec![], new_edges.clone(), UpdateKind::Insert);

            // Delete the new added edge from graph
            let (_,_,t3) = bse.update_graph(vec![], new_edges, UpdateKind::Delete);

            println!("{},{},{},{},{}", nodes_num, edges_num, t1, t2, t3);
        }
    }
}