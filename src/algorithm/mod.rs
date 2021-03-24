use super::{structure::graph::Graph, utils};
use std::thread;
use std::sync::Arc;

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
    // let t = (100. * (k as u32).pow(3) as f64 / eps.powf(2.0)) as u32;
    let t = 1000;

    let g_arc = Arc::new(g);
    let mut handles = Vec::new();
    for _j in 1..t + 1 {
        let g_loc = Arc::clone(&g_arc);
        let handle = thread::spawn(move || {
            let bernoulli_mapping = utils::create_bernoulli(k);
            g_loc.compute_walk_sum(k, bernoulli_mapping).sorted().coeffs()[0]
        });
        println!("{}/{}", _j, t);
        handles.push(handle);
    }

    let mut xs = Vec::new();
    for h in handles {
        let res = h.join().unwrap();
        xs.push(res);
    }

    let sum: f64 = xs.iter().sum();
    let denom = (utils::factorial(k) * t as u128) as f64;
    (sum / denom).abs()
}


pub fn c_sync(g: Graph, k: usize, eps: f64) -> f64 {
    let t = (100. * (k as u32).pow(3) as f64 / eps.powf(2.0)) as u32;

    let mut xs = Vec::new();
    for _j in 1..t + 1 {
        let bernoulli_mapping = utils::create_bernoulli(k);
        let x = g.compute_walk_sum(k, bernoulli_mapping).sorted().coeffs()[0];
        // println!("{}/{}", _j, t);
        xs.push(x);
    }

    let sum: f64 = xs.iter().sum();
    let denom = (utils::factorial(k) * t as u128) as f64;
    (sum / denom).abs()
}

#[cfg(test)]
mod tests {
    use crate::algorithm;
    use crate::structure::graph::Graph;
    use std::time::Instant;

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
        let eps = 0.4;

        let now = Instant::now();
//        let res = algorithm::c(g, k, eps);
        println!("algorihm c took: {}s", now.elapsed().as_secs());

        /* 
        let g2 = Graph::from_graph6("src/data/test_graphs/path10.g6");
        let now2 = Instant::now();
        let res2 = algorithm::c_sync(g2, k, eps);
        println!("algorihm c_sync took: {}s", now2.elapsed().as_secs());
        */

        /*
        let p = 18.;
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
        */
    }
}
