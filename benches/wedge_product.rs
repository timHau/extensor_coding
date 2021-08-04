mod utils;

use extensor_coding::{extensor::bitvec, extensor::dense_hashmap};
use plotters::style;
use rand::Rng;
use std::time::Instant;

fn rand_coeffs_and_basis_vec(n: i32) -> (Vec<i64>, Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let coeffs: Vec<i64> = (0..n).map(|_| rng.gen::<i16>() as i64).collect();
    let basis: Vec<Vec<u8>> = (1..=n).map(|i| vec![i as u8]).collect();
    (coeffs, basis)
}

fn bench_bitvec(num_iter: u64) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_basis = 31;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis_vec(n);
            let ext_1 = bitvec::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis_vec(n);
            let ext_2 = bitvec::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
            let elapsed = now.elapsed().as_nanos() as f64;

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    times
}

fn bench_hashmap(num_iter: u64) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_basis = 31;

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis_vec(n);
            let ext_1 = dense_hashmap::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis_vec(n);
            let ext_2 = dense_hashmap::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
            let elapsed = now.elapsed().as_nanos() as f64;

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
    }

    times
}

fn main() {
    let num_iter = 300;

    let times_bitvec = bench_bitvec(num_iter);
    let times_hashmap = bench_hashmap(num_iter);

    let result = vec![
        ("bitvec".to_string(), style::RED, times_bitvec),
        ("dense_hashmap".to_string(), style::BLUE, times_hashmap),
    ];
    let _ = utils::plot_results(
        "wedge product comparison (vector)",
        (
            ("Anzahl von Basiselementen", 0f32..31f32),
            ("Laufzeit (in ns)", 0f32..1000000f32),
        ),
        0,
        "benches/output/wedge_prod",
        &result,
    );
}
