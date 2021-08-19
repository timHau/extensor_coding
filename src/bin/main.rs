use extensor_coding::{algorithm, graph::Graph};

fn main() {
    let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");
    let k = 4;
    let eps = 0.1;
    let res = algorithm::c(g, k, eps);
    println!("res: {}", res)
}
