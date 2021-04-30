pub mod algorithm;
mod extensor;
mod graph;
mod matrix;
mod utils;

use graph::Graph;

fn main() {
    let g = Graph::from_graph6("src/data/test_graphs/path6.g6");
    let k = 3;
    let eps = 0.1;
    let res = algorithm::c(g, k, eps);
    println!("res: {}", res);
}
