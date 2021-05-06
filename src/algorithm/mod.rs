use crate::{graph::Graph, utils};
use num_traits::Zero;

/// # Algorithm U
///
/// Given an Graph `g` and i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
pub fn u(g: &Graph, k: usize) -> bool {
    let vandermonde_mapping = utils::create_vandermonde(g.num_vert(), k);
    let res = g.compute_walk_sum(k, vandermonde_mapping);
    !res.is_zero()
}

/// # Algorithm C
///
pub fn c(g: Graph, k: usize, eps: f64) -> f64 {
    let t = (100.0 * (k as f64).powf(3.0) / eps.powf(2.0)) as u64;

    let mut sum = 0;
    let mut x_s = Vec::new();
    for j in 0..t {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert(), k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeff = v_j.coeffs()[0];
        sum += coeff;
        x_s.push(coeff);
        println!("{} / {}", j, t);
    }

    sum as f64 / (utils::factorial(k) * t) as f64
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
        let g = Graph::from_graph6("src/data/test_graphs/path3.g6");
        let k = 3;
        let eps = 0.3;
        let now = std::time::Instant::now();
        let res = algorithm::c(g, k, eps);
        println!("algorihm c took: {}s", now.elapsed().as_secs());

        let p = 2.;
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
