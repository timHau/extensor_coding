mod utils;

use extensor_coding::{algorithm, graph::Graph};
use plotters::style;
use std::time::Instant;

fn bench_c(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_k = 9;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for k in 2..=max_k {
            let g = Graph::from_graph6("src/data/path10.g6");
            let eps = 0.9;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    utils::join_runs(times)
}

fn main() {
    let num_iter = 10;
    let times_algo_c = bench_c(num_iter);

    let result = vec![("algorithm c".to_string(), style::RED, times_algo_c)];
    let _ = utils::plot_results(
        "algorithm c (dense_hashmap, sparse matrix)",
        (("k", 2f32..9f32), ("Zeit (in ns)", 0f32..600000f32)),
        "benches/output/algo.png",
        &result,
    );
}
