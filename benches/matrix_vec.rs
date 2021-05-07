use extensor_coding::{
    matrix::naive, matrix::naive_parallel, matrix::sparse_hash, matrix::sparse_triples,
};
use rand::Rng;
use std::time::Instant;

fn rand_vec(n: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..1)).collect()
}

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

    join_runs(times)
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

    join_runs(times)
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

    join_runs(times)
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

    join_runs(times)
}

fn main() {
    let num_iter = 50;
    let times_naive = bench_naive(num_iter);
    // let times_naive_parallel = bench_naive_parallel(num_iter);
    let times_triples = bench_triples(num_iter);
    let times_hash = bench_hash(num_iter);

    println!("naive: {:?}", times_naive);
    // println!("naive_parallel: {:?}", times_naive_parallel);
    println!("triples: {:?}", times_triples);
    println!("hash: {:?}", times_hash);
}
