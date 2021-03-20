use super::structure::extensor::ExTensor;
use std::{collections::HashSet, hash::Hash};
use rand::{Rng, distributions::{Distribution, Uniform}};

type F = Box<dyn Fn(usize) -> ExTensor>;
type G = Box<dyn Fn(usize, usize) -> f64>;

pub fn get_permutation_to_sort<T>(v: &[T]) -> Vec<usize>
where
    T: std::cmp::Ord,
{
    let mut perm: Vec<_> = (0..v.len()).collect();
    perm.sort_by_cached_key(|&i| &v[i]);
    perm
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

/// given k, create a lifted vandermonde coding that takes v as input
pub fn create_vandermonde(k: usize) -> (F, G) {
    let f_vert = move |v: usize| -> ExTensor {
        let coeffs: Vec<f64> = (0..k).map(|i| v.pow(i as u32) as f64).collect();
        let basis: Vec<Vec<i32>> = (0..k).map(|i| vec![i as i32]).collect();
        ExTensor::from(coeffs, basis).lifted()
    };
    let f_edge = |_v: usize, _w: usize| 1.0;
    (Box::new(f_vert), Box::new(f_edge))
}

/// given k, create a lifted bernoulli coding
pub fn create_bernoulli(k: usize) -> (F, G) {
    let f_vert = move |v: usize| -> ExTensor {
        let mut rng = rand::thread_rng();
        // create a uniform random variable that is either 0 or 1
        let mut unif = Uniform::from(0..2);
        let coeffs: Vec<f64> = (0..k).map(|i| {
            // transform the variable to be either -1 or +1
            let mut rand_val = unif.sample(&mut rng);
            if rand_val == 0 {
                rand_val = -1;
            }
            rand_val as f64
        }).collect();
        let basis: Vec<Vec<i32>> = (0..k).map(|i| vec![i as i32]).collect();
        ExTensor::from(coeffs, basis).lifted()
    };
    let f_edge = |_v: usize, _w: usize| 1.0;
    (Box::new(f_vert), Box::new(f_edge))
}


#[cfg(test)]
mod tests {
    use crate::utils::create_bernoulli;

    #[test]
    fn bernoulli() {
        let k = 3;
        let (f_vert, f_edge) = create_bernoulli(k);
        let vert_val = f_vert(4);
        assert_eq!(f_edge(3, 4), 1.0, "function on edge is constant 1");
        for coeff in vert_val.coeffs() {
            assert!(coeff == 1.0 || coeff == -1.0, "coefficients are either +1 or -1");
        }
    }
}
