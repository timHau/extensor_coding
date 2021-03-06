use crate::{graph::Graph, utils};
use num_traits::Zero;

/// # Algorithm U
///
/// Given an Graph `g` and an i32 `k` as input, such that the number of `k`-paths in
/// G is 0 or 1, decide if there is a `k`-path in `g`
///
/// Arguments:
///
/// * `g`: Graph, where to decide if k-path exist
/// * `k`: length of path
///
/// The algorithm is from [Brand, Dell and Husfeldt](https://arxiv.org/pdf/1804.09448.pdf)
pub fn u(g: &Graph, k: usize) -> bool {
    let vandermonde_mapping = utils::create_vandermonde(g.num_vert, k);
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
/// Arguments:
///
/// * `g`: Graph, where to find k-path
/// * `k`: length of path
/// * `eps`: approximation accuracy
///
/// The algorithm is from [Brand, Dell and Husfeldt](https://arxiv.org/pdf/1804.09448.pdf)
pub fn c(g: Graph, k: usize, eps: f64) -> f64 {
    let mut step = 1;
    let mut mean = f64::INFINITY;
    let mut values = Vec::new();
    let mut means = Vec::new();

    while step < ((k as f64).powf(2.0) / eps.powf(2.0)) as u32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let n = step as f64;
        mean = utils::mean(&values);
        means.push(mean);
        let std_dev = utils::std_dev(&means);

        let t_val = utils::t_value(step - 1);
        if ((mean - t_val * std_dev / n.sqrt() > (1.0 - eps) * mean) || std_dev == 0.0) && step > 30
        {
            return mean;
        }
        step += 1;
    }

    mean
}

/// only used for benchmarking, returns the number of iterations
pub fn c_count_iterations(g: Graph, k: usize, eps: f64) -> u32 {
    let mut step = 1u32;
    let mut values = Vec::new();
    let mut means = Vec::new();

    while step < 100 * ((k as f64).powf(3.0) / eps.powf(2.0)) as u32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let n = step as f64;
        let mean = utils::mean(&values);
        means.push(mean);
        let std_dev = utils::std_dev(&means);
        let t_val = utils::t_value(step - 1);

        println!("step: {}, mean: {}", step, mean);
        if (mean - t_val * std_dev / n.sqrt() > (1.0 - eps) * mean) || (std_dev == 0.0 && step > 20)
        {
            return step;
        }
        step += 1;
    }

    step
}

// only used for debugging / benchmarking. Returns "history" of values
pub fn c_values_naive(g: Graph, k: usize, eps: f64) -> Vec<f64> {
    let mut step = 1;
    let mut values = Vec::new();
    let mut means = Vec::new();

    while step < ((k as f64).powf(2.0) / eps.powf(2.0)) as u32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let mean = utils::mean(&values);
        means.push(mean);
        let std_dev = utils::std_dev(&means);

        println!("mean: {}, std_dev: {}, step: {}", mean, std_dev, step);
        step += 1;
    }

    means
}

// only used for debugging / benchmarking. Returns "history" of values
pub fn c_values_t_test(g: Graph, k: usize, eps: f64) -> Vec<f64> {
    let mut step = 1;
    let mut values = Vec::new();
    let mut means = Vec::new();

    while step < ((k as f64).powf(2.0) / eps.powf(2.0)) as u32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let n = step as f64;
        let mean = utils::mean(&values);
        means.push(mean);
        let std_dev = utils::std_dev(&means);
        let t_val = utils::t_value(step - 1);

        println!("mean: {}, std_dev: {}, step: {}", mean, std_dev, step);
        if ((mean - t_val * std_dev / n.sqrt()) > (1.0 - eps) * mean)
            || (std_dev == 0.0 && step > 20)
        {
            return means;
        }
        step += 1;
    }

    means
}

// only used for debugging / benchmarking. Returns "history" of values
pub fn c_values_std_dev(g: Graph, k: usize, eps: f64) -> Vec<f64> {
    let mut step = 1;
    let mut values = Vec::new();
    let mut means = Vec::new();
    let mut std_dev = f64::INFINITY;

    while std_dev > eps {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let mean = utils::mean(&values);
        means.push(mean);
        std_dev = utils::std_dev(&means);

        println!("mean: {}, std_dev: {}, step: {}", mean, std_dev, step);
        step += 1;
    }

    means
}

pub fn c_std_dev(g: Graph, k: usize, _eps: f64) -> Vec<f64> {
    let mut step = 1;
    let mut values = Vec::new();
    let mut means = Vec::new();
    let mut std_devs = Vec::new();

    while step < 4000 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert, k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = (coeffs.abs() as f64) / denom;
        values.push(x_j);

        let mean = utils::mean(&values);
        means.push(mean);
        let std_dev = utils::std_dev(&means);
        std_devs.push(std_dev);

        println!("std_dev: {}, step: {}", std_dev, step);

        step += 1;
    }

    std_devs
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::{algorithm, utils};

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
        let res = algorithm::c(g, k, eps);
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
        let eps = 0.9;
        let res = algorithm::c(g, k, eps);
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

    #[test]
    fn c_3() {
        let g = Graph::from_graph6("src/data/path6.g6");
        let k = 3;
        let eps = 0.5;
        let p = 8.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
        let res = algorithm::c(g, k, eps);
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
    fn c_4() {
        let g = Graph::from_graph6("src/data/path6.g6");
        let k = 4;
        let eps = 0.5;
        let p = 6.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
        let res = algorithm::c(g, k, eps);
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
    fn c_tree() {
        // test algorithm c on the following binary tree
        //          o
        //         / \
        //        /   \
        //       o     o
        //      / \   / \
        //     /   \ /   \
        //    o    o o    o
        // edges are directed and point on "down"
        let g = Graph::from(
            7,
            vec![
                0, 1, 0, 0, 1, 0, 0, // root
                0, 0, 1, 1, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 1, 1, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
            ],
        );
        let k = 2;
        let eps = 0.2;
        let expect = 6.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
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
    fn c_tree_2() {
        // test algorithm c on the following binary tree
        //          o
        //         / \
        //        /   \
        //       o     o
        //      / \   / \
        //     /   \ /   \
        //    o    o o    o
        // edges are directed and point on "down"
        let g = Graph::from(
            7,
            vec![
                0, 1, 0, 0, 1, 0, 0, // root
                0, 0, 1, 1, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 1, 1, //
                0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, //
            ],
        );
        let k = 3;
        let eps = 0.4;
        let expect = 4.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
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
    fn c_triangle() {
        // test algorithm c on the following binary tree
        //          o
        //         / \
        //        /   \
        //       o --- o
        // edges form a "circle"
        let g = Graph::from(
            3,
            vec![
                0, 1, 0, // first node
                0, 0, 1, // second node
                1, 0, 0,
            ],
        );
        let k = 2;
        let eps = 0.5;
        let expect = 3.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
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
    fn c_triangle_2() {
        // test algorithm c on the following binary tree
        //          1
        //         / \
        //        /   \
        //       2 --- 3
        // edges form a "circle"
        let g = Graph::from(
            3,
            vec![
                0, 1, 0, // first node
                0, 0, 1, // second node
                1, 0, 0,
            ],
        );
        let k = 3;
        let eps = 0.5;
        let expect = 3.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
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
    fn c_triangle_3() {
        // test algorithm c on the following binary tree
        //          1
        //         / \
        //        /   \
        //       2 --- 3
        // edges form a "circle"
        let g = Graph::from(
            3,
            vec![
                0, 1, 0, // first node
                0, 0, 1, // second node
                1, 0, 0,
            ],
        );
        let k = 4;
        let eps = 0.2;
        let res = algorithm::c(g, k, eps);
        assert_eq!(
            res, 0.0,
            "algorithm c vanishes when path contains a vertex twice"
        );
    }

    #[test]
    fn c_graph() {
        // test algorithm x on this (undirected) graph
        // 1            5
        //  \          /
        //   2 ------ 4
        //  /          \
        // 3            6
        let g = Graph::from(
            6,
            vec![
                0, 1, 0, 0, 0, 0, //
                1, 0, 1, 1, 0, 0, //
                0, 1, 0, 0, 0, 0, //
                0, 1, 0, 0, 1, 1, //
                0, 0, 0, 1, 0, 0, //
                0, 0, 0, 1, 0, 0, //
            ],
        );
        let k = 3;
        let eps = 0.3;
        let res = algorithm::c(g, k, eps);
        let expect = 12.0;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
    fn c_graph_2() {
        // test algorithm x on this (undirected) graph
        // 1            5
        //  \          /
        //   2 ------ 4
        //  /          \
        // 3            6
        let g = Graph::from(
            6,
            vec![
                0, 1, 0, 0, 0, 0, //
                1, 0, 1, 1, 0, 0, //
                0, 1, 0, 0, 0, 0, //
                0, 1, 0, 0, 1, 1, //
                0, 0, 0, 1, 0, 0, //
                0, 0, 0, 1, 0, 0, //
            ],
        );
        let k = 4;
        let eps = 0.5;
        let res = algorithm::c(g, k, eps);
        let expect = 8.0;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
    fn c_graph_3() {
        // test algorithm x on this (undirected) graph
        // 1-------2-------3
        //  \     / \     /
        //   \   /   \   /
        //     4 ----- 5
        //      \     /
        //       \   /
        //         6
        let g = Graph::from(
            6,
            vec![
                0, 1, 0, 1, 0, 0, //
                1, 0, 1, 1, 1, 0, //
                0, 1, 0, 0, 1, 0, //
                1, 1, 0, 0, 1, 1, //
                0, 1, 1, 1, 0, 1, //
                0, 0, 0, 1, 1, 0, //
            ],
        );
        let k = 2;
        let eps = 0.3;
        let res = algorithm::c(g, k, eps);
        let expect = 18.0;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
    fn c_graph_4() {
        // test algorithm x on this (undirected) graph
        // 1-------2-------3
        //  \     / \     /
        //   \   /   \   /
        //     4 ----- 5
        //      \     /
        //       \   /
        //         6
        let g = Graph::from(
            6,
            vec![
                0, 1, 0, 1, 0, 0, //
                1, 0, 1, 1, 1, 0, //
                0, 1, 0, 0, 1, 0, //
                1, 1, 0, 0, 1, 1, //
                0, 1, 1, 1, 0, 1, //
                0, 0, 0, 1, 1, 0, //
            ],
        );
        let k = 4;
        let eps = 0.5;
        let res = algorithm::c(g, k, eps);
        let expect = 66.0;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
    fn complete_graph_10_3_path() {
        // Test algorithm c on complete graph with 10 vertices
        let g = Graph::from_graph6("src/data/K10.g6");
        let k = 3;
        let eps = 0.8;
        let res = algorithm::c(g, k, eps);
        let n = 10;
        let expect = utils::factorial(n) as f64 / utils::factorial(n - k) as f64;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
    fn complete_graph_20_3_path() {
        // Test algorithm c on complete graph with 20 vertices
        let g = Graph::from_graph6("src/data/K20.g6");
        let k = 3;
        let eps = 0.9;
        let res = algorithm::c(g, k, eps);
        let n = 20;
        let expect = utils::factorial(n) as f64 / utils::factorial(n - k) as f64;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
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
