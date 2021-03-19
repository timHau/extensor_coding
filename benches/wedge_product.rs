extern crate extensor_coding as ec;
#[macro_use]
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ec::{
    algorithm,
    structure::graph::Graph,
};

pub fn criterion_benchmark(c: &mut Criterion)
{
    let g = Graph::from_graph6("src/data/test_graphs/path3.g6");
    let k = 3;
    c.bench_function("main", |b| b.iter(|| algorithm::u(&g, k)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);