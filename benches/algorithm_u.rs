extern crate extensor_coding as ec;
#[macro_use]
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ec::{algorithm, structure::graph::Graph};

pub fn criterion_benchmark(c: &mut Criterion) {
    let g = Graph::from_graph6("src/data/test_graphs/path3.g6");
    let k = 3;
    c.bench_function("main", |b| b.iter(|| algorithm::u(&g, k)));
}

pub fn algorithm_u_path_4(c: &mut Criterion) {
    let g = Graph::from_graph6("src/data/test_graphs/path4.g6");
    let k = 4;
    c.bench_function("main", |b| b.iter(|| algorithm::u(&g, k)));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = algorithm_u_path_3, algorithm_u_path_4,
}
criterion_main!(benches);
