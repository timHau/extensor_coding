use extensor_coding::{algorithm, graph::Graph};

fn main() {
    // let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");
    let g = Graph::from_graph6("src/data/K20.g6");
    let k = 4;
    let eps = 0.1;
    let res = algorithm::c(g, k, eps);
    println!("res: {}", res)
}
