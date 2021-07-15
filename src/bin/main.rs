use extensor_coding::{algorithm, graph::Graph};
use std::time::Instant;

fn main() {
    let g = Graph::from_tsv("src/data/out.moreno_kangaroo_kangaroo");
    // let g = Graph::from_graph6("src/data/path10.g6");
    let k = 2;
    let eps = 0.2;

    let now = Instant::now();
    let res = algorithm::c(g, k, eps);
    println!("took {}ms", now.elapsed().as_millis());

    println!("res: {}", res);
    println!("{} {} {}", (1.0 - eps) * 18.0, res, (1.0 + eps) * 18.0);
}
