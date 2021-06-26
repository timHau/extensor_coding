mod utils;

use extensor_coding::{algorithm, graph::Graph};
use indicatif::{ProgressBar, ProgressStyle};
use plotters::style;
use std::time::Instant;

fn bench_c(num_iter: u64, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_k = 10;
    let bar = ProgressBar::new(num_iter);
    bar.set_style(prog_style.clone());

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for k in 2..=max_k {
            let g = Graph::from_graph6("src/data/path10.g6");
            let eps = 0.9;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
        bar.inc(1);
    }
    bar.finish();

    times
}

fn bench_c_grow_n(num_iter: u64, k: usize, p: f64, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_n = 50usize;
    let bar = ProgressBar::new(num_iter);
    bar.set_style(prog_style.clone());

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for n in 2..=max_n {
            let g = utils::rand_graph(n, p);
            let eps = 0.9;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
        bar.inc(1);
    }
    bar.finish();

    times
}

fn main() {
    let num_iter = 1;

    let prog_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("=>-");

    /*
    let times_algo_c = bench_c(num_iter, &prog_style);

    let result = vec![("algorithm c".to_string(), style::RED, times_algo_c)];
    let _ = utils::plot_results(
        "algorithm c (dense_hashmap, sparse matrix)",
        (("k", 2f32..11f32), ("Zeit (in ns)", 0f32..600000f32)),
        2,
        "benches/output/algo",
        &result,
    );
    */

    let times_algo_c_n_k_2 = bench_c_grow_n(10, 2, 0.5, &prog_style);
    let times_algo_c_n_k_3 = bench_c_grow_n(10, 3, 0.5, &prog_style);
    let times_algo_c_n_k_4 = bench_c_grow_n(10, 4, 0.5, &prog_style);
    let result_n = vec![
        ("k = 2".to_string(), style::BLUE, times_algo_c_n_k_2),
        ("k = 3".to_string(), style::GREEN, times_algo_c_n_k_3),
        ("k = 4".to_string(), style::RED, times_algo_c_n_k_4),
    ];
    /*
    let _ = utils::plot_results(
        "random graph with n vertices (p=0.5)",
        (("n", 2f32..50f32), ("Zeit (in ns)", 0f32..600000f32)),
        2,
        "benches/output/algo_n",
        &result_n,
    );
    */

    let times_algo_c_n_k_2_p = bench_c_grow_n(10, 2, 0.2, &prog_style);
    let times_algo_c_n_k_3_p = bench_c_grow_n(10, 3, 0.2, &prog_style);
    let times_algo_c_n_k_4_p = bench_c_grow_n(10, 4, 0.2, &prog_style);
    let result_n = vec![
        ("k = 2".to_string(), style::BLUE, times_algo_c_n_k_2_p),
        ("k = 3".to_string(), style::GREEN, times_algo_c_n_k_3_p),
        ("k = 4".to_string(), style::RED, times_algo_c_n_k_4_p),
    ];
    /*
    let _ = utils::plot_results(
        "random graph with n vertices (p=0.2)",
        (("n", 2f32..50f32), ("Zeit (in ns)", 0f32..600000f32)),
        2,
        "benches/output/algo_n",
        &result_n,
    );
    */
}
