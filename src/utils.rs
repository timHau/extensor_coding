#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use rand::distributions::{Distribution, Uniform};

/// given k, create a lifted vandermonde coding that takes v as input
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

/// given k, create a lifted bernoulli coding
pub(crate) fn create_bernoulli(n: usize, k: usize) -> Vec<ExTensor> {
    let mut res = Vec::with_capacity(n);
    res.reserve(n);

    for v in 1..=n {
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

pub(crate) fn factorial(k: usize) -> u64 {
    let mut res = 1;
    for i in 1..=k as u64 {
        res *= i;
    }
    res
}

pub(crate) fn binomial(n: usize, k: usize) -> u64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

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

#[cfg(test)]
mod tests {
    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;

    use crate::matrix::sparse_triples::Matrix;

    use crate::utils::{
        binomial, contains_element, create_bernoulli, create_vandermonde, factorial,
        has_intersection,
    };

    /*
    #[test]
    fn vandermonde() {
        let k = 5;
        let (f_vert, f_edge) = create_vandermonde(k);
        assert_eq!(f_edge(3, 4), 1.0, "function on edge is constant 1");
        let vert_val_1 = f_vert(1);
        let vert_val_2 = f_vert(2);
        let vert_val_3 = f_vert(3);
        let vert_val_4 = f_vert(4);
        let vert_val_5 = f_vert(5);
        let prod = vert_val_1 * vert_val_2 * vert_val_3 * vert_val_4 * vert_val_5;
        let res = ExTensor::new(&[82944], &[vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
        assert_eq!(prod, res, "lifted vandermonde");
    }

    #[test]
    fn bernoulli() {
        let k = 5;
        let (f_vert, f_edge) = create_bernoulli(k);
        let vert_val = f_vert(4);
        assert_eq!(f_edge(3, 4), 1.0, "function on edge is constant 1");
        for coeff in vert_val.coeffs() {
            assert!(
                coeff == 1 || coeff == -1,
                "coefficients are either +1 or -1"
            );
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

    #[test]
    fn binom() {
        let bc = binomial(5, 2);
        assert_eq!(bc, 10, "5 choose 2");
    }

    #[test]
    fn tmp() {
        let k = 2;
        let (f_vert, _) = create_bernoulli(k);

        let m = Matrix::new(3, 3, vec![0, 1, 0, 0, 0, 1, 0, 0, 0]).add_coding(&f_vert);
        println!("m: {}", m);

        let b = vec![f_vert(1), f_vert(2), f_vert(3)];

        println!("b:");
        for bv in b.iter() {
            println!("{}", bv);
        }

        let mut r = &m * b;
        for _i in 1..(k - 1) {
            println!("\n iteration: {}", _i);
            println!("m: {}", m);

            r = &m * r;
        }

        let prod = f_vert(1) * f_vert(2);

        println!("\n");
        for rv in r.iter() {
            println!("rv: {}", rv)
        }

        println!(
            "\nsum: {:?}",
            r.clone()
                .into_iter()
                .fold(ExTensor::new(&[], &[]), |acc, v| acc + v),
        );
    }
    */
}
