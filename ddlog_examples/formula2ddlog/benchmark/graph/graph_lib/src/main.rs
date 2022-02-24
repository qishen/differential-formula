use std::fs;
use std::path::Path;
use graph_lib::*;

fn gen_formula_program(domain_str: String, nodes_num: usize, edges_num: usize) -> String {
    let mut program_model_str = "".to_string();
    let graph = Graph::gen_random_graph(nodes_num, edges_num);
    let edges = graph.edges();
    for (src, dst) in edges.iter() { 
        let term_str = format!("Edge(Node({}), Node({})).\n", src, dst); 
        program_model_str += &term_str[..];
    }
    let program_str = format!("{}\n model m of Graph {{\n {} \n}}", domain_str, program_model_str);
    return program_str;
}

// Usage:
// cargo run --release -- <NODE> <EDGE> <TARGET> debug
fn main() {
    // println!("Print out arguments: {:?}", std::env::args());
    let nodes_num: usize = std::env::args().nth(1).unwrap_or("0".to_string()).parse().unwrap_or(0);
    let edges_num: usize = std::env::args().nth(2).unwrap_or("0".to_string()).parse().unwrap_or(0);
    let target: String = std::env::args().nth(3).unwrap_or("ddlog".to_string());
    // By default, do not print out delta.
    let debug = std::env::args().nth(4).unwrap_or("nodebug".to_string()) == "debug";

    // Print out the header
    println!("nodes,edges,init,del,add");

    if target == "formula" {
        let path = Path::new("../graph_template.4ml");
        let domain_str = fs::read_to_string(path).unwrap();
        let program_str = gen_formula_program(domain_str, nodes_num, edges_num);
        println!("{}", program_str);
        let write_path = format!("../files/graph_n{}_e{:04}.4ml", nodes_num, edges_num);
        fs::write(write_path, program_str).unwrap();
    } else if target == "formula_bench" {
        let path = Path::new("../graph_template.4ml");
        let domain_str = fs::read_to_string(path).unwrap();
        for i in 0 .. 50 {
            let edge_interval = edges_num;
            let edges_num = edge_interval * (i + 1);
            let program_str = gen_formula_program(domain_str.clone(), nodes_num, edges_num);
            println!("{}", program_str);
            let write_path = format!("../files/graph_n{}_e{:04}.4ml", nodes_num, edges_num);
            fs::write(write_path, program_str).unwrap();
        }
    } else if target == "ddlog" {
        let graph = Graph::gen_random_graph(nodes_num, edges_num);
        let nodes = graph.nodes();
        let edges = graph.edges();
        let mut bse = DDLogGraph::new(debug).unwrap();
        let (_,_,t) = bse.init_graph(nodes, edges);
        println!("Nodes: {}, Edges: {}, Running time: {} milli seconds", nodes_num, edges_num, t);
    } else if target == "ddlog_bench" {
        for i in 0 .. 50 {
            let edge_interval = edges_num;
            let edges_num = edge_interval * (i + 1);
            let graph = Graph::gen_random_graph(nodes_num, edges_num);
            let nodes = graph.nodes();
            let edges = graph.edges();
            let mut bse = DDLogGraph::new(debug).unwrap();
            let (_,_, t1) = bse.init_graph(nodes, edges);
            
            // For specific graph algorithm like transitive closure, adding one edge to the graph
            // could potentially double the running time so we delete some edges first and then 
            // add them back to show the benefits of incremental updates 
            let mut one_percent_edges = graph.edges();
            one_percent_edges.truncate(edges_num / 100);
            
            // Delete some existing edges
            let (_,_,t2) = bse.update_graph(vec![], one_percent_edges.clone(), UpdateKind::Delete);

            // Add the deleted edges back again
            let (_,_,t3) = bse.update_graph(vec![], one_percent_edges, UpdateKind::Insert);

            println!("{},{},{},{},{}", nodes_num, edges_num, t1, t2, t3);
        }
    }
}