use super::{structure::graph::Graph, utils};

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
pub fn c(g: &Graph, k: usize, eps: f64) -> f64 {
    // let t = (100. * (k as u32).pow(3) as f64 / eps.powf(2.0)) as u32;
    let t = 200;

    let mut xs = Vec::new();
    for _j in 1..t + 1 {
        let bernoulli_mapping = utils::create_bernoulli(k);
        let x_j = g.compute_walk_sum(k, bernoulli_mapping).sorted().coeffs()[0];
        xs.push(x_j);
    }

    let sum: f64 = xs.iter().sum();
    let denom = (utils::factorial(k) * t as u128) as f64;
    sum / denom
}

#[cfg(test)]
mod tests {
    use crate::algorithm;
    use crate::structure::graph::Graph;

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
        let eps = 0.2;
        let res = algorithm::c(&g, k, eps);
        let lower_bound = (1. - eps) * res.abs();
        let upper_bound = (1. + eps) * res.abs();
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }
}
