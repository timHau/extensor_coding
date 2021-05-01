pub mod algorithm;
mod extensor;
mod graph;
mod matrix;
mod utils;

use graph::Graph;
use std::time::Instant;

fn main() {
    let g = Graph::from_graph6("src/data/test_graphs/path6.g6");
    let k = 2;
    let eps = 0.3;

    let now = Instant::now();
    let res = algorithm::c(g, k, eps);
    println!("took {}ms", now.elapsed().as_millis());

    println!("res: {}", res);
}
