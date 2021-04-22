use crate::extensor::dense_hashmap::ExTensor;
use rand::distributions::{Distribution, Uniform};
use std::u128;

type F = Box<dyn Fn(usize) -> ExTensor>;
type G = Box<dyn Fn(usize, usize) -> f64>;

/// given k, create a lifted vandermonde coding that takes v as input
pub fn create_vandermonde(k: usize) -> (F, G) {
    let f_vert = move |v: usize| -> ExTensor {
        let coeffs: Vec<f64> = (0..k).map(|i| v.pow(i as u32) as f64).collect();
        let basis: Vec<Vec<u32>> = (1..=k).map(|i| vec![i as u32]).collect();
        ExTensor::new(&coeffs, &basis).lift(k)
    };
    let f_edge = |_v: usize, _w: usize| 1.0;
    (Box::new(f_vert), Box::new(f_edge))
}

/// given k, create a lifted bernoulli coding
pub fn create_bernoulli(k: usize) -> (F, G) {
    let f_vert = move |_v: usize| -> ExTensor {
        let mut rng = rand::thread_rng();
        // create a uniform random variable that is either 0 or 1
        let unif = Uniform::from(0..2);
        let coeffs: Vec<f64> = (0..k)
            .map(|_i| {
                // transform the variable to be either -1 or +1
                let mut rand_val = unif.sample(&mut rng);
                if rand_val == 0 {
                    rand_val = -1;
                }
                rand_val as f64
            })
            .collect();
        let basis: Vec<Vec<u32>> = (1..=k).map(|i| vec![i as u32]).collect();
        ExTensor::new(&coeffs, &basis).lift(k)
    };
    let f_edge = |_v: usize, _w: usize| 1.0;
    (Box::new(f_vert), Box::new(f_edge))
}

pub fn factorial(k: usize) -> u128 {
    let mut res: u128 = 1;
    for i in 1..(k as u32 + 1) {
        res *= i as u128;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::utils::{create_bernoulli, factorial};

    #[test]
    fn bernoulli() {
        let k = 5;
        let (f_vert, f_edge) = create_bernoulli(k);
        let vert_val = f_vert(4);
        assert_eq!(f_edge(3, 4), 1.0, "function on edge is constant 1");
        for coeff in vert_val.coeffs() {
            assert!(
                coeff == 1.0 || coeff == -1.0,
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
        assert_eq!(r1, 6 as u128, "3!");
        assert_eq!(r2, 24 as u128, "4!");
        assert_eq!(r3, 5040 as u128, "7!");
        assert_eq!(r4, 3628800 as u128, "10!");
    }
}
