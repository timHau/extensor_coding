mod utils;

use extensor_coding::{algorithm, graph::Graph};
use plotters::style;

fn bench_convergence(num_iter: u64) -> Vec<Vec<f64>> {
    let mut all_values = vec![];

    for _j in 0..num_iter {
        let g = Graph::from_graph6("src/data/path10.g6");
        let k = 4;
        let eps = 0.2;
        let values = algorithm::c_values_t_test(g, k, eps);
        all_values.push(values)
    }

    all_values
}

fn main() {
    let num_iter = 10;
    let values = bench_convergence(num_iter);
    let result = vec![("path 10, k = 4".to_string(), style::RED, values.clone())];

    let _ = utils::plot_results(
        "Konvergenz, Algorithm c",
        (("Anzahl Iteration", 0f32..400f32), ("Mean", 5f32..20f32)),
        0,
        "benches/output/convergence",
        &result,
    );

    let result = vec![("path 10, k = 4".to_string(), style::BLUE, values.clone())];
    let _1 = utils::plot_results_histogram(
        "Konvergenz, Algorithm c",
        (("Werte", 5u32..20u32), ("Häufigkeit", 0u32..130u32)),
        "benches/output/convergence_histogram",
        &result,
    );
}
