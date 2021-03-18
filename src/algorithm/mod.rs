use super::utils;
use super::structure::extensor::ExTensor;
use super::structure::graph::Graph;

/// # Algorithm U
///
/// Given an Graph `g` and i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
fn u(g: Graph, k: usize) -> bool {
    fn f_edge(u: usize, v: usize) -> f64 {
        1.0
    }
    let res = g.compute_walk_sum(k,utils::create_vandermonde(k) , f_edge);
    let zero = ExTensor::zero();
    res == zero
}
