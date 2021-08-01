mod utils;

use extensor_coding::{algorithm, graph::Graph};
use plotters::style;
use std::time::Instant;

fn bench_c(num_iter: u64, path_str: &str) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_k = 9;

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
    }

    times
}

fn bench_c_grow_n(num_iter: u64, k: usize, p: f64) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_n = 80usize;

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
    }

    times
}

fn count_iterations(num_iter: u64, g: Graph) -> Vec<Vec<f64>> {
    let max_k = 5;
    let mut iterations = Vec::new();

    for _j in 0..num_iter {
        let mut iter = Vec::new();
        for k in 2..=max_k {
            let eps = 0.7;
            let n = algorithm::c_count_iterations(g.clone(), k, eps);
            iter.push(n as f64);
            println!("k: {}, n: {}", k, n);
        }
        iterations.push(iter);
    }

    iterations
}

fn iterations_eps(num_iter: u64) -> Vec<Vec<f64>> {
    let mut iterations = Vec::new();

    for _j in 0..num_iter {
        let mut iter = Vec::new();
        for i in 1..=10 {
            let eps = 1.0 / (i as f64);
            let g = utils::rand_graph(20, 0.5);
            let n = algorithm::c_count_iterations(g, 4, eps);
            iter.push(n as f64);
            println!(" n: {}", n);
        }
        iterations.push(iter);
    }

    iterations
}

fn main() {
    /*
    let g_rand = utils::rand_graph(200, 0.5);
    let num_iterations_rand = count_iterations(1, g_rand);

    let g_gplus = Graph::from_tsv("src/data/out.ego-gplus");
    let num_iterations_gplus = count_iterations(1, g_gplus);

    let result = vec![
        (
            "random graph (200 vertices, p = 0.5)".to_string(),
            style::RED,
            num_iterations_rand,
        ),
        (
            "(real world) graph google plus".to_string(),
            style::GREEN,
            num_iterations_gplus,
        ),
    ];
    let _ = utils::plot_results(
        "number of iterations",
        (("k", 2f32..11f32), ("iterations", 1f32..200f32)),
        2,
        "benches/output/iterations",
        &result,
    );
    */

    let iter_eps = iterations_eps(1);
    let results = vec![("".to_string(), style::RED, iter_eps)];
    let _ = utils::plot_results(
        "iterations vs epsilon",
        (("epsilon^-1", 1f32..10f32), ("iterations", 1f32..200f32)),
        1,
        "benches/output/iterations_eps",
        &results,
    );

    /*
    //   let times_algo_c_10 = bench_c(2, "src/data/path10.g6");
    //    let times_algo_c_100 = bench_c(2, "src/data/path100.g6");
    let times_algo_c_tutte = bench_c(1, "src/data/path100.g6");

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

    let times_algo_c_n_k_8 = bench_c_grow_n(10, 2, 0.8);
    let times_algo_c_n_k_4 = bench_c_grow_n(10, 2, 0.4);
    let times_algo_c_n_k_2 = bench_c_grow_n(10, 2, 0.2);
    let times_algo_c_n_k_1 = bench_c_grow_n(10, 2, 0.1);
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
