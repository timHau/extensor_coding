use extensor_coding::{extensor::bitvec, extensor::dense_hashmap};
use rand::Rng;
use std::time::Instant;

fn rand_coeffs_and_basis(n: i32) -> (Vec<i64>, Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let coeffs: Vec<i64> = (0..n).map(|_| rng.gen::<i16>() as i64).collect();
    let basis: Vec<Vec<u8>> = (1..=n).map(|i| vec![i as u8]).collect();
    (coeffs, basis)
}

fn join_runs(runs: Vec<Vec<u128>>) -> Vec<f64> {
    let mut res = vec![0.0; runs.len()];

    for tv in runs.iter() {
        for (i, v) in tv.iter().enumerate() {
            res[i] += *v as f64;
        }
    }

    res.iter()
        .map(|t| *t / runs.len() as f64)
        .collect::<Vec<f64>>()
}

fn bench_bitvec(num_iter: i32) -> Vec<f64> {
    let mut times = Vec::new();
    let max_basis = 30;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis(n);
            let ext_1 = bitvec::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis(n);
            let ext_2 = bitvec::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
            let elapsed = now.elapsed().as_millis();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter)
    }

    join_runs(times)
}

fn bench_hashmap(num_iter: i32) {
    let mut times = Vec::new();
    let max_basis = 30;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        println!("{} / {}", _j, num_iter);

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis(n);
            let ext_1 = dense_hashmap::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis(n);
            let ext_2 = dense_hashmap::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
            let elapsed = now.elapsed().as_millis();

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter)
    }
}

fn main() {
    let num_iter = 50;
    let times_bitvec = bench_bitvec(num_iter);
    let times_hashmap = bench_hashmap(num_iter);

    println!("times: {:?}", times_bitvec);
}
