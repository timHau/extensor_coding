use extensor_coding::{algorithm, graph::Graph};
use std::time::Instant;

fn main() {
    let g = Graph::from_tsv("src/data/out.moreno_kangaroo_kangaroo");
    let k = 4;
    let eps = 0.1;

    let now = Instant::now();
    let res = algorithm::c(g, k, eps);
    println!("took {}ms", now.elapsed().as_millis());

    println!("res: {}", res);
    println!("{} {} {}", (1.0 - eps) * 10.0, res, (1.0 + eps) * 10.0);
}
