use extensor_coding::{algorithm, graph::Graph};
use std::time::Instant;

fn join_runs(runs: Vec<Vec<u128>>) -> Vec<f64> {
    let mut res = vec![0.0; runs[0].len()];

    for tv in runs.iter() {
        for (i, v) in tv.iter().enumerate() {
            res[i] += *v as f64;
        }
    }

    res.iter()
        .map(|t| *t / (runs.len() as f64))
        .collect::<Vec<f64>>()
}

fn bench_c(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_k = 10;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for k in 2..=max_k {
            let g = Graph::from_graph6("src/data/test_graphs/path100.g6");
            let eps = 0.9;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    join_runs(times)
}

fn main() {
    let num_iter = 10;
    let times_algo_c = bench_c(num_iter);

    println!("algorithm c: {:?}", times_algo_c);
}
