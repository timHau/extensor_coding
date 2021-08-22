use extensor_coding::{algorithm, graph::Graph};
use std::fs;
use std::time::Instant;

mod utils;

pub(crate) fn mean(values: &Vec<f64>) -> f64 {
    match values.len() {
        0 => 0.0,
        _ => values.iter().sum::<f64>() / (values.len() as f64),
    }
}

fn bench_rust(num_iter: u64) -> Vec<f64> {
    let mut times = vec![];
    let max_k = 3;

    for k in 2..=max_k {
        let mut times_per_iter = vec![];

        for _j in 0..num_iter {
            let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");
            let eps = 0.5;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
            println!("[rust] k: {}", k);
        }

        times.push(mean(&times_per_iter));
    }

    times
}

fn main() {
    let times = bench_rust(1);

    let mut data = "".to_owned();
    for (i, t) in times.iter().enumerate() {
        data.push_str(&format!("{}, {} \n", i + 2, t));
    }

    fs::write("benches/output/bench_k_rust.txt", data).expect("Unable to write file");
}
