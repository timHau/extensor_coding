use extensor_coding::{algorithm, graph::Graph};
use std::time::Instant;

mod utils;

fn bench_rust(num_iter: u64) -> Vec<Vec<f64>> {
    let mut times = vec![];
    let max_k = 9;

    for _j in 0..num_iter {
        let mut times_per_iter = vec![];

        for k in 2..=max_k {
            let g = Graph::from_tsv("../src/data/out.brunson_revolution_revolution");
            let eps = 0.5;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
            println!("[rust] k: {}", k);
        }

        times.push(times_per_iter);
    }

    times
}

fn main() {}
