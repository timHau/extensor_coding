mod utils;

use extensor_coding::{algorithm, graph::Graph};
use indicatif::{ProgressBar, ProgressStyle};
use plotters::{prelude::IntoLogRange, style};
use std::time::Instant;

fn bench_c(num_iter: u64, path_str: &str, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_k = 9;
    let bar = ProgressBar::new(num_iter);
    bar.set_style(prog_style.clone());

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for k in 2..=max_k {
            let g = Graph::from_graph6(path_str);
            let eps = 0.9;

            let now = Instant::now();
            let _ = algorithm::c(g, k, eps);
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
            println!("k: {}", k);
        }

        times.push(times_per_iter);
        bar.inc(1);
    }
    bar.finish();

    times
}

fn bench_c_grow_n(num_iter: u64, k: usize, p: f64, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_n = 80usize;
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

fn count_iterations() -> Vec<u32> {
    let max_k = 4;
    let mut iterations = Vec::new();

    for k in 2..=max_k {
        let g = utils::rand_graph(1000, 0.3);
        let eps = 0.9;

        let n = algorithm::c_count_iterations(g, k, eps);
        iterations.push(n);
    }

    iterations
}

fn main() {
    let prog_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("=>-");

    let num_iterations = count_iterations();
    let result = (
        "number of iterations".to_string(),
        style::RED,
        num_iterations,
    );
    let _ = utils::box_plot(
        "number of iterations",
        (("k", 2u32..11u32), ("iterations", 1u32..200u32)),
        2,
        "benches/output/iterations",
        &result,
    );

    /*
    //   let times_algo_c_10 = bench_c(2, "src/data/path10.g6", &prog_style);
    //    let times_algo_c_100 = bench_c(2, "src/data/path100.g6", &prog_style);
    let times_algo_c_tutte = bench_c(1, "src/data/path100.g6", &prog_style);

    let result = vec![
        /*       (
            "algorithm c (path 10)".to_string(),
            style::RED,
            times_algo_c_10,
        ),
        (
            "algorithm c (path 100)".to_string(),
            style::BLUE,
            times_algo_c_100,
        ), */
        (
            "algorithm c (tutte graph)".to_string(),
            style::GREEN,
            times_algo_c_tutte,
        ),
    ];
    let _ = utils::plot_results_log(
        "algorithm c (dense_hashmap, sparse matrix)",
        (
            ("k", 2f32..11f32),
            ("Zeit (in ns)", (0.1f32..800000f32).log_scale()),
        ),
        2,
        "benches/output/algo",
        &result,
    );

    let times_algo_c_n_k_8 = bench_c_grow_n(10, 2, 0.8, &prog_style);
    let times_algo_c_n_k_4 = bench_c_grow_n(10, 2, 0.4, &prog_style);
    let times_algo_c_n_k_2 = bench_c_grow_n(10, 2, 0.2, &prog_style);
    let times_algo_c_n_k_1 = bench_c_grow_n(10, 2, 0.1, &prog_style);
    let result_n = vec![
        (
            "p = 0.8, k = 2".to_string(),
            style::BLUE,
            times_algo_c_n_k_8,
        ),
        (
            "p = 0.4, k = 2".to_string(),
            style::GREEN,
            times_algo_c_n_k_4,
        ),
        ("p = 0.2, k = 2".to_string(), style::RED, times_algo_c_n_k_2),
        (
            "p = 0.1, k = 2".to_string(),
            style::RGBColor(120, 200, 228),
            times_algo_c_n_k_1,
        ),
    ];
    let _ = utils::plot_results(
        "algorithm c on random graph with n vertices, probability p",
        (("n", 2f32..80f32), ("Zeit (in ns)", 0f32..120f32)),
        2,
        "benches/output/algo_n_0_5",
        &result_n,
    );
    */
}
