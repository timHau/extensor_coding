mod utils;

use extensor_coding::{
    matrix::naive, matrix::naive_parallel, matrix::sparse_hash, matrix::sparse_triples,
};
use plotters::style;
use rand::Rng;
use std::time::Instant;

fn rand_vec(n: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..1)).collect()
}

fn bench_naive(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_n = 500;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_n {
            let m = naive::Matrix::new(n, n, rand_vec((n * n) as i32));
            let v = rand_vec(n as i32);

            let now = Instant::now();
            let _ = &m * v;
            let elapsed = now.elapsed().as_nanos();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    utils::join_runs(times)
}

fn bench_naive_parallel(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_n = 500;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_n {
            let m = naive_parallel::Matrix::new(n, n, rand_vec((n * n) as i32));
            let v = rand_vec(n as i32);

            let now = Instant::now();
            let _ = &m * v;
            let elapsed = now.elapsed().as_nanos();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    utils::join_runs(times)
}

fn bench_triples(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_n = 500;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_n {
            let m = sparse_triples::Matrix::new(n, n, rand_vec((n * n) as i32));
            let v = rand_vec(n as i32);

            let now = Instant::now();
            let _ = &m * v;
            let elapsed = now.elapsed().as_nanos();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    utils::join_runs(times)
}

fn bench_hash(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_n = 500;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_n {
            let m = sparse_hash::Matrix::new(n, n, rand_vec((n * n) as i32));
            let v = rand_vec(n as i32);

            let now = Instant::now();
            let _ = &m * v;
            let elapsed = now.elapsed().as_nanos();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    utils::join_runs(times)
}

fn main() {
    let num_iter = 50;
    let times_naive = bench_naive(num_iter);
    // let times_naive_parallel = bench_naive_parallel(num_iter);
    let times_triples = bench_triples(num_iter);
    let times_hash = bench_hash(num_iter);

    let result = vec![
        ("naive".to_string(), style::RED, times_naive),
        ("triples".to_string(), style::GREEN, times_triples.clone()),
        ("hashmap".to_string(), style::BLUE, times_hash.clone()),
    ];
    let _ = utils::plot_results(
        "matrix vec prod",
        (("n", 0f32..500f32), ("Zeit (in ns)", 0f32..500000f32)),
        "benches/output/matrix_vec.png",
        &result,
    );

    let sparse_result = vec![
        ("triples".to_string(), style::GREEN, times_triples),
        ("hashmap".to_string(), style::BLUE, times_hash),
    ];
    let _ = utils::plot_results(
        "sparse matrix vec prod",
        (("n", 0f32..500f32), ("Zeit (in ns)", 0f32..12000f32)),
        "benches/output/matrix_vec_sparse.png",
        &sparse_result,
    );
}
