use std::collections::HashMap;

use crate::{graph::Graph, utils};
use itertools::Itertools;
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
/// Arguments:
///
/// * `g`: Graph, where to find k-path
/// * `k`: length of path
/// * `eps`: approximation accuracy
///
/// The algorithm is from [Brand, Dell and Husfeldt](https://arxiv.org/pdf/1804.09448.pdf)
pub fn c(g: Graph, k: usize, eps: f64) -> f64 {
    let mut t = 1;
    let mut sum = 0.0;
    let mut ssum = 0.0;

    while t < 4 * (k as f64 / eps.powf(2.0)) as i32 {
        let bernoulli_mapping = utils::create_bernoulli(g.num_vert(), k);
        let v_j = g.compute_walk_sum(k, bernoulli_mapping);
        let coeffs = if v_j.coeffs().is_empty() {
            0.0
        } else {
            v_j.coeffs()[0] as f64
        };
        let denom = utils::factorial(k) as f64;
        let x_j = coeffs.abs() as f64 / denom;

        sum += x_j;
        ssum += x_j * x_j;
        t += 1;

        let n = t as f64;
        let mean = sum / n;
        let std_dev = ((ssum - mean * mean * n) / (n - 1.0)).sqrt();
        let t_val = utils::t_value(t - 1);

        println!(
            "mean: {}, std_dev: {}, x_j: {}, coeffs: {:?}",
            mean, std_dev, x_j, coeffs
        );
        if (mean - t_val * std_dev / n.sqrt() > (1.0 - eps) * mean) || (std_dev == 0.0 && t > 20) {
            return mean;
        }
    }

    sum / (t as f64)
}

pub fn color_coding(g: Graph, k: usize) -> f64 {
    let mut dp_table = HashMap::new();
    let g = g.color_coding(k);
    let color_set: Vec<usize> = (1..=k).collect();

    for set in color_set.iter().powerset() {
        match set.len() {
            0 => continue,
            1 => {
                for (v, color) in g.vert_data.iter().enumerate() {
                    if set[0] == color {
                        dp_table.insert((set.clone(), v), 1);
                    } else {
                        dp_table.insert((set.clone(), v), 0);
                    }
                }
            }
            _ => {
                for (v, color) in g.vert_data.iter().enumerate() {
                    let set_minus_color: Vec<&usize> =
                        set.clone().into_iter().filter(|c| *c != color).collect();

                    let mut value = 0;
                    for u in g.neighbors_of(v).iter() {
                        let neighbor_value =
                            dp_table.get(&(set_minus_color.clone(), *u)).unwrap_or(&0);
                        value = value + *neighbor_value;
                    }

                    dp_table.insert((set_minus_color.clone(), v), value);
                }
            }
        }
    }
    println!("dp_table: {:?}", dp_table);

    let mut res = 0.0;
    for ((s, _v), value) in dp_table.into_iter() {
        println!("s {:?}, val: {}", s, value);
        if s.len() == k - 1 {
            res += value as f64;
        }
    }
    res
}

pub fn color_coding_rec(g: Graph, k: usize) -> f64 {
    let mut sum = 0.0;
    let num_iter = f64::exp(k as f64);
    for _ in 0..num_iter as u32 {
        let mut res = 0.;
        let g = g.color_coding(k);
        for (v, color) in g.vert_data.iter().enumerate() {
            res += color_coding_step(&g, v, *color, (1..=k).collect()) as f64;
        }
        res *= 2.0;
        sum += res;
    }

    sum / (num_iter as f64)
}

fn color_coding_step(g: &Graph, v: usize, color: usize, s: Vec<usize>) -> u32 {
    if s.len() == 1 {
        if s[0] == color {
            return 1;
        } else {
            return 0;
        }
    }

    let mut c = 0;
    for u in g.neighbors_of(v).iter() {
        let s_minus_col: Vec<usize> = s.clone().into_iter().filter(|c| *c != color).collect();
        c += color_coding_step(g, *u, g.vert_data[*u], s_minus_col);
    }

    c
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
        let eps = 0.5;
        let res = algorithm::c(g, k, eps);
        let p = 2.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
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
        let eps = 0.2;
        let expect = 4.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
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
        let eps = 0.2;
        let expect = 3.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }

    #[test]
    fn c_triangle_2() {
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
        let k = 3;
        let eps = 0.2;
        let expect = 3.;
        let lower_bound = (1. - eps) * expect;
        let upper_bound = (1. + eps) * expect;
        let res = algorithm::c(g, k, eps);
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }

    #[test]
    fn c_triangle_3() {
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
        let k = 4;
        let eps = 0.2;
        let res = algorithm::c(g, k, eps);
        assert_eq!(
            res, 0.0,
            "algorithm c vanishes when path contains a vertex twice"
        );
    }

    #[test]
    fn color_coding() {
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 2;
        let eps = 0.9;
        let p = 4.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
        let res = algorithm::color_coding_rec(g, k);
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }

    #[test]
    fn color_coding_2() {
        let source = "src/data/path6.g6";
        let g = Graph::from_graph6(source);
        let k = 2;
        let eps = 0.5;
        let p = 10.;
        let lower_bound = (1. - eps) * p;
        let upper_bound = (1. + eps) * p;
        let res = algorithm::color_coding_rec(g, k);
        assert!(
            lower_bound <= res.abs() && res.abs() <= upper_bound,
            "randomized counting algorithm c is inside bounds"
        );
    }

    #[test]
    fn dp_color_coding() {
        let g = Graph::from_graph6("src/data/path6.g6");
        let k = 3;
        let res = algorithm::color_coding(g, k);
        println!("res: {}", res);
    }
}
