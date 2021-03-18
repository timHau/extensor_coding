mod algorithm;
mod structure;
mod utils;

use structure::graph::Graph;

fn main() {
    let g = Graph::from_graph6("src/data/test_graphs/path6.g6");
    let res = algorithm::u(g, 6);
    println!("antwort: {}", res);
}
