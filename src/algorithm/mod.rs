use super::structure::extensor::ExTensor;
use super::structure::graph::Graph;
use super::utils;

/// # Algorithm U
///
/// Given an Graph `g` and i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
fn u(g: Graph, k: usize) -> bool {
    let vandermonde_mapping = utils::create_vandermonde(k);
    let res = g.compute_walk_sum(k, vandermonde_mapping);
    let zero = ExTensor::zero();
    res == zero
}
