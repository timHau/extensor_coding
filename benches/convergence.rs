mod utils;

use extensor_coding::{algorithm, graph::Graph};
use plotters::style;

fn bench_convergence_t_test(num_iter: u64, g: Graph) -> Vec<Vec<f64>> {
    let mut all_values = vec![];

    for _j in 0..num_iter {
        let k = 4;
        let eps = 0.2;
        let values = algorithm::c_values_t_test(g.clone(), k, eps);
        all_values.push(values)
    }

    all_values
}

fn bench_convergence_naive(num_iter: u64, g: Graph) -> Vec<Vec<f64>> {
    let mut all_values = vec![];

    for _j in 0..num_iter {
        let k = 4;
        let eps = 0.2;
        let values = algorithm::c_values_naive(g.clone(), k, eps);
        all_values.push(values)
    }

    all_values
}

fn main() {
    let num_iter_t_test = 1;
    let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");

    let values_t_test = bench_convergence_t_test(num_iter_t_test, g.clone());
    let result_t_test = vec![(
        "graph: brunson_revolution, k = 4".to_string(),
        style::RED,
        values_t_test.clone(),
    )];

    let _t_test = utils::plot_results(
        "Konvergenz, Algorithm c (t-test)",
        (
            ("Anzahl Iteration", 0f32..50f32),
            ("Ergebnis (Algorithmus C)", 150f32..300f32),
        ),
        0,
        "benches/output/convergence_t_test",
        &result_t_test,
    );

    let result_t_test_hist = vec![(
        "path 10, k = 4".to_string(),
        style::BLUE,
        values_t_test.clone(),
    )];
    let _1 = utils::plot_results_histogram(
        "Konvergenz, Algorithm c Histogram (t-test)",
        (("Werte", 150u32..300u32), ("Häufigkeit", 0u32..50u32)),
        "benches/output/convergence_histogram_t_test",
        &result_t_test_hist,
    );

    let num_iter_naive = 15;
    let values_naive = bench_convergence_naive(num_iter_naive, g.clone());
    let result_naive = vec![(
        "graph: brunson_revolution, k = 4".to_string(),
        style::RED,
        values_naive.clone(),
    )];

    let _naive = utils::plot_results(
        "Konvergenz, Algorithm c (naive)",
        (
            ("Anzahl Iteration", 0f32..400f32),
            ("Ergebnis (Algorithmus C)", 150f32..300f32),
        ),
        0,
        "benches/output/convergence_naive",
        &result_naive,
    );

    let result_naive_hist = vec![(
        "path 10, k = 4".to_string(),
        style::BLUE,
        values_naive.clone(),
    )];
    let _2 = utils::plot_results_histogram(
        "Konvergenz, Algorithm c Histogram (naive)",
        (("Werte", 150u32..300u32), ("Häufigkeit", 0u32..150u32)),
        "benches/output/convergence_histogram_naive",
        &result_naive_hist,
    );
}
