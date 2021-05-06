#![allow(dead_code)]

pub mod algorithm;
mod extensor;
mod graph;
mod matrix;
mod utils;

use graph::Graph;
use std::time::Instant;

fn main() {
    let g = Graph::from_graph6("src/data/test_graphs/path10.g6");
    let k = 3;
    let eps = 0.9;

    let now = Instant::now();
    let res = algorithm::c(g, k, eps);
    println!("took {}ms", now.elapsed().as_millis());

    println!("res: {}", res);
}
