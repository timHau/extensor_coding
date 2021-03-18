use super::structure::extensor::ExTensor;
use std::collections::HashSet;
use std::hash::Hash;

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

/// given k create a vandermonde coding that takes v as input
pub fn create_vandermonde(k: usize) -> Box<dyn Fn(usize) -> ExTensor> {
    let v = move |v: usize| -> ExTensor {
        let coeffs: Vec<f64> = (0..k).map(|i| v.pow(i as u32) as f64).collect();
        let basis: Vec<Vec<i32>> = (0..k).map(|i| vec![i as i32]).collect();
        ExTensor::from(coeffs, basis)
    };
    Box::new(v)
}
