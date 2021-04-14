use crate::{graph::Graph, utils};
use num_traits::Zero;
use std::sync::mpsc;
use std::time::Instant;
use threadpool::ThreadPool;

/// # Algorithm U
///
/// Given an Graph `g` and i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
pub fn u(g: &Graph, k: usize) -> bool {
    let vandermonde_mapping = utils::create_vandermonde(k);
    let res = g.compute_walk_sum(k, vandermonde_mapping);
    !res.is_zero()
}

/// # Algorithm C
///
pub fn c(g: Graph, k: usize, eps: f64) -> f64 {
    let t = (100. * (k as u32).pow(3) as f64 / eps.powf(2.0)) as u32;

    let now = Instant::now();
    let mut xs = Vec::new();
    for _j in 0..t {
        let bernoulli_mapping = utils::create_bernoulli(k);
        let x_j = g.compute_walk_sum(k, bernoulli_mapping).coeffs()[0];
        println!("{}/{}", _j, t);
        xs.push(x_j);
    }
    println!("in c: {}", now.elapsed().as_millis());

    let sum: f64 = xs.iter().sum();
    let denom = (utils::factorial(k) * t as u128) as f64;
    (sum / denom).abs()
}

pub fn c_parallel(g: Graph, k: usize, eps: f64) -> f64 {
    let t = (100. * (k as u32).pow(3) as f64 / eps.powf(2.0)) as u32;

    let n_workers = 20;
    let pool = ThreadPool::new(n_workers);
    let (sender, receiver) = mpsc::channel();

    for _j in 0..t {
        let sender = sender.clone();
        let k = k.clone();
        let g = g.clone();
        let t = t.clone();
        let _j = _j.clone();
        pool.execute(move || {
            let bernoulli_mapping = utils::create_bernoulli(k);
            let x_j = g.compute_walk_sum(k, bernoulli_mapping).coeffs()[0];
            println!("{}/{}", _j, t);
            sender.send(x_j).unwrap();
        });
    }

    let sum: f64 = receiver.iter().take(t as usize).sum();
    let denom = (utils::factorial(k) * t as u128) as f64;
    (sum / denom).abs()
}

#[cfg(test)]
mod tests {
    use crate::algorithm;
    use crate::graph::Graph;

    #[test]
    fn u_3() {
        let g = Graph::from_graph6("src/data/test_graphs/path3.g6");
        let k = 3;
        let res = algorithm::u(&g, k);
        assert_eq!(res, true, "algorithm u on 3 path graph");
    }

    #[test]
    fn u_4() {
        let g = Graph::from_graph6("src/data/test_graphs/path4.g6");
        let k = 4;
        let res = algorithm::u(&g, k);
        assert_eq!(res, true, "algorithm u on 4 path graph");
    }

    #[test]
    fn u_4_3() {
        let g = Graph::from_graph6("src/data/test_graphs/path3.g6");
        let k = 4;
        let res = algorithm::u(&g, k);
        assert_eq!(res, false, "no 4 path in a 3 path graph");
    }

    #[test]
    fn c() {
        let g = Graph::from_graph6("src/data/test_graphs/path10.g6");
        let k = 3;
        let eps = 0.2;
        let now = std::time::Instant::now();
        let res = algorithm::c_parallel(g, k, eps);
        println!("algorihm c took: {}s", now.elapsed().as_secs());

        let p = 16.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
        println!(
            "lower: {}, res: {}, upper: {}",
            lower_bound, res, upper_bound
        );
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }
}
