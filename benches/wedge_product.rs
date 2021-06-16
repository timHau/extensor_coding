mod utils;

use extensor_coding::{extensor::bitvec, extensor::dense_hashmap};
use indicatif::{ProgressBar, ProgressStyle};
use plotters::style;
use rand::Rng;
use std::time::Instant;

fn rand_coeffs_and_basis(n: i32) -> (Vec<i64>, Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let coeffs: Vec<i64> = (0..n).map(|_| rng.gen::<i16>() as i64).collect();
    let basis: Vec<Vec<u8>> = (1..=n).map(|i| vec![i as u8]).collect();
    (coeffs, basis)
}

fn bench_bitvec(num_iter: u64, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_basis = 31;
    let bar = ProgressBar::new(num_iter);
    bar.set_style(prog_style.clone());

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis(n);
            let ext_1 = bitvec::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis(n);
            let ext_2 = bitvec::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
            let elapsed = now.elapsed().as_millis() as f64;

            times_per_iter.push(elapsed);
        }

        times.push(times_per_iter);
        bar.inc(1);
    }
    bar.finish();

    times
}

fn bench_hashmap(num_iter: u64, prog_style: &ProgressStyle) -> Vec<Vec<f64>> {
    let mut times = Vec::new();
    let max_basis = 80;
    let bar = ProgressBar::new(num_iter);
    bar.set_style(prog_style.clone());

    for _j in 0..num_iter {
        let mut times_per_iter = Vec::new();

        for n in 1..=max_basis {
            let (coeffs_1, basis_1) = rand_coeffs_and_basis(n);
            let ext_1 = dense_hashmap::ExTensor::new(&coeffs_1, &basis_1);

            let (coeffs_2, basis_2) = rand_coeffs_and_basis(n);
            let ext_2 = dense_hashmap::ExTensor::new(&coeffs_2, &basis_2);

            let now = Instant::now();
            let _ = ext_1 * ext_2;
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
    let num_iter = 50;

    let prog_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("=>-");

    let times_bitvec = bench_bitvec(num_iter, &prog_style);
    let times_hashmap = bench_hashmap(num_iter, &prog_style);

    let result = vec![
        ("bitvec".to_string(), style::RED, times_bitvec),
        ("dense_hashmap".to_string(), style::BLUE, times_hashmap),
    ];
    let _ = utils::plot_results(
        "wedge product comparison",
        (
            ("Nummer von Basiselementen", 0f32..80f32),
            ("Laufzeit (in ms)", 0f32..40f32),
        ),
        0,
        "benches/output/wedge_prod",
        &result,
    );
}
