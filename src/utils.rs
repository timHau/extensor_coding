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
