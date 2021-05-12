#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use rand::distributions::{Distribution, Uniform};

/// ## create_vandermonde
///
/// Given `k`, create a lifted vandermonde coding
/// `n` is the number of vertices in the graph
/// For every vertex `v` in the Graph create a exterior Tensor
// v_i ↦ (i^0, i^1, ...,  i^(k-1))
pub(crate) fn create_vandermonde(n: usize, k: usize) -> Vec<ExTensor> {
    let mut res = Vec::with_capacity(n);
    res.reserve(n);

    for v in 1..=n {
        let coeffs: Vec<i64> = (0..k).map(|i| v.pow(i as u32) as i64).collect();
        let basis: Vec<Vec<u8>> = (1..=k).map(|i| vec![i as u8]).collect();
        let col = ExTensor::new(&coeffs, &basis).lift(k);
        res.push(col);
    }

    res
}

/// ## create_bernoulli
///
/// given k, create a lifted bernoulli coding
/// `n` is the number of vertices in the graph
/// For every vertex `v` in the Graph create a exterior Tensor
// v_i ↦ (±1, ±1, ...,  ±1)
pub(crate) fn create_bernoulli(n: usize, k: usize) -> Vec<ExTensor> {
    let mut res = Vec::with_capacity(n);
    res.reserve(n);

    for _v in 1..=n {
        let coeffs: Vec<i64> = (0..k)
            .map(|_i| {
                let mut rng = rand::thread_rng();
                let unif = Uniform::from(0..2);
                // create a uniform random variable that is either 0 or 1
                // transform the variable to be either -1 or +1
                let mut rand_val = unif.sample(&mut rng);
                if rand_val == 0 {
                    rand_val = -1;
                }
                rand_val as i64
            })
            .collect();
        let basis: Vec<Vec<u8>> = (1..=k).map(|i| vec![i as u8]).collect();
        let col = ExTensor::new(&coeffs, &basis).lift(k);
        res.push(col);
    }

    res
}

/// ## file_n_from
///
/// given a `path_str` which is the path to a graph6 file as a string, it opens the file and returns
/// the file with `n` which is the number of vertices in that graph.
/// It works with .g6 and .s6 files, with and without headers. If the file is not found it will panic.
pub(crate) fn file_n_from(path_str: &str) -> (Vec<u8>, usize) {
    // read file if it exists
    let mut file = std::fs::read(path_str).expect(".graph6 input file not found");

    let mut _n = 0;

    let has_sparse_header =
        file.len() > 10 && std::str::from_utf8(&file[..11]).unwrap() == ">>sparse6<<";
    let has_graph_header =
        file.len() > 9 && std::str::from_utf8(&file[..10]).unwrap() == ">>graph6<<";
    let is_sparse = file[0] as char == ':' || has_sparse_header;

    if !is_sparse {
        if has_graph_header {
            _n = (file[10] - 63) as usize;
            file = file[11..].to_vec();
        } else {
            _n = (file[0] - 63) as usize;
            file = file[1..].to_vec();
        }
    } else if has_sparse_header {
        _n = (file[12] - 63) as usize;
        file = file[13..].to_vec();
    } else {
        _n = (file[1] - 63) as usize;
        file = file[2..].to_vec();
    }

    if _n > 62 {
        let n1 = ((file[0] - 63) as i32) << 12;
        let n2 = ((file[1] - 63) as i32) << 6;
        let n3 = (file[2] - 63) as i32;
        _n = (n1 + n2 + n3) as usize;
        file = file[3..].to_vec();
    }

    (file, _n)
}

/// ## factorial
///
/// calculates k!
pub(crate) fn factorial(k: usize) -> u64 {
    let mut res = 1;
    for i in 1..=k as u64 {
        res *= i;
    }
    res
}

/// ## contains_element
///
/// determine if a sorted vec `v` contains `target`
pub(crate) fn contains_element(v: &Vec<u8>, target: &u8) -> bool {
    let mut l = 0usize;
    let mut r = v.len() - 1;

    while l <= r {
        let m = (l + r) / 2;
        if &v[m] < target {
            l = m + 1;
        } else if &v[m] > target {
            if m == 0 {
                return false;
            }
            r = m - 1;
        } else {
            return true;
        }
    }

    false
}

/// ## has_intersection
///
/// determine if two sorted (!!) vecs have at least one common element
pub(crate) fn has_intersection(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    // both vecs are sorted, so we can use binary search
    for val in a.iter() {
        if contains_element(b, val) {
            return true;
        }
    }

    false
}

pub(crate) fn t_value(df: i32) -> f64 {
    match df {
        0..=4 => 3.747,
        5..=8 => 2.896,
        9..=16 => 2.583,
        17..=32 => 2.457,
        33..=64 => 2.390,
        65..=128 => 2.358,
        _ => 2.326,
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;

    use crate::utils::{
        contains_element, create_bernoulli, create_vandermonde, factorial, has_intersection,
    };

    #[test]
    #[cfg(feature = "extensor_dense_hashmap")]
    fn vandermonde() {
        let k = 5;
        let n = 5;
        let coding = create_vandermonde(n, k);
        let vert_val_1 = &coding[0];
        let vert_val_2 = &coding[1];
        let vert_val_3 = &coding[2];
        let vert_val_4 = &coding[3];
        let vert_val_5 = &coding[4];
        let prod = &(&(&(vert_val_1 * vert_val_2) * vert_val_3) * vert_val_4) * vert_val_5;
        let res = ExTensor::new(&[82944], &[vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
        assert_eq!(prod, res, "lifted vandermonde");
    }

    #[test]
    fn bernoulli() {
        let k = 3;
        let n = 5;
        let coding = create_bernoulli(n, k);
        for _i in 0..n {
            let vert_val = &coding[4];
            for coeff in vert_val.coeffs() {
                assert!(
                    coeff == 1 || coeff == -1,
                    "coefficients are either +1 or -1"
                );
            }
        }
    }

    #[test]
    fn facto() {
        let r1 = factorial(3);
        let r2 = factorial(4);
        let r3 = factorial(7);
        let r4 = factorial(10);
        assert_eq!(r1, 6, "3!");
        assert_eq!(r2, 24, "4!");
        assert_eq!(r3, 5040, "7!");
        assert_eq!(r4, 3628800, "10!");
    }

    #[test]
    fn contains_elem() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let res = contains_element(&v, &2);
        assert_eq!(res, true);
        assert_eq!(contains_element(&v, &9), false);
    }

    #[test]
    fn intersect() {
        let v_1 = vec![1, 2, 3, 4, 5, 6];
        let v_2 = vec![6, 7, 8, 9, 10, 11];
        let res = has_intersection(&v_1, &v_2);
        assert_eq!(res, true);
        let v_3 = vec![7, 8, 9, 10, 11, 12];
        let res_2 = has_intersection(&v_1, &v_3);
        assert_eq!(res_2, false);
    }
}
