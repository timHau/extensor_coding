use crate::{graph::Graph, utils};
use num_traits::Zero;

/// # Algorithm U
///
/// Given an Graph `g` and an i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
///
/// The algorithm is from [Brand, Dell and Husfeldt](https://arxiv.org/pdf/1804.09448.pdf)
pub fn u(g: &Graph, k: usize) -> bool {
    let vandermonde_mapping = utils::create_vandermonde(g.num_vert(), k);
    let res = g.compute_walk_sum(k, vandermonde_mapping);
    !res.is_zero()
}

/// # Algorithm C
///
/// Given an Graph `g` and an i32 `k` as input, approximately count the number
/// of `k`-paths that are contained in `g`. Which means, Algorithm c
/// produces a value (f64) `X` such that with probability of `99%` the number of `k`-paths
/// in `g` satisfies
/// ```not-a-test
/// (1 - eps) * number of `k`-paths <= X <= (1 + eps) * number of `k`-paths
/// ```
///
/// The algorithm is from [Brand, Dell and Husfeldt](https://arxiv.org/pdf/1804.09448.pdf)
pub fn c(g: Graph, k: usize, eps: f64) -> f64 {
    let mut t = 0;
    let mut sum = 0.0;
    let mut ssum = 0.0;

    while t < 4 * (k as f64 / eps.powf(2.0)) as i32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert(), k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeff = v_j.coeffs()[0].abs() as f64;
        let denom = utils::factorial(k) as f64;
        let x_j = coeff / denom;

        sum += x_j;
        ssum += x_j * x_j;
        t += 1;

        println!("t: {}", t);

        let n = t as f64;
        if n > 10.0 {
            let mean = sum / n;
            let std_dev = ((ssum - mean * mean * n) / (n - 1.0)).sqrt();
            let t_val = utils::t_value(t - 1);

            println!("mean: {}, std_dev: {}", mean, std_dev);

            if mean - t_val * std_dev / n.sqrt() > (1.0 - eps) * mean {
                return mean;
            }
        }
    }

    sum / (t as f64)
}

pub fn color_coding(g: Graph, k: usize) {
    let _g = g.color_coding(k);
}

#[cfg(test)]
mod tests {
    use crate::algorithm;
    use crate::graph::Graph;

    #[test]
    fn u_3() {
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 3;
        let res = algorithm::u(&g, k);
        assert_eq!(res, true, "algorithm u on 3 path graph");
    }

    #[test]
    fn u_4() {
        let g = Graph::from_graph6("src/data/path4.g6");
        let k = 4;
        let res = algorithm::u(&g, k);
        assert_eq!(res, true, "algorithm u on 4 path graph");
    }

    #[test]
    fn u_4_3() {
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 4;
        let res = algorithm::u(&g, k);
        assert_eq!(res, false, "no 4 path in a 3 path graph");
    }

    #[test]
    fn c() {
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 2;
        let eps = 0.5;
        let now = std::time::Instant::now();
        let res = algorithm::c(g, k, eps);
        println!("algorihm c took: {}s", now.elapsed().as_secs());

        let p = 4.;
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

    #[test]
    fn c_2() {
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 3;
        let eps = 0.5;
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
