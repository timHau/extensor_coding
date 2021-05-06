#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use num_traits::identities::{One, Zero};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Matrix<T> {
    nrows: usize,
    ncols: usize,
    data: HashMap<usize, Vec<(usize, T)>>,
}

impl<T> Matrix<T>
where
    T: Clone + One + Zero,
{
    pub(crate) fn new(nrows: usize, ncols: usize, values: Vec<T>) -> Self {
        assert_eq!(
            values.len(),
            nrows * ncols,
            "dimensons of values does not match"
        );

        let mut data = HashMap::new();

        for (i, val) in values.into_iter().enumerate() {
            if !val.is_zero() {
                let row_index = i / ncols;
                let col_index = i % nrows;

                let row_vals = data.entry(row_index).or_insert(Vec::new());
                row_vals.push((col_index, val));
            }
        }

        Matrix { nrows, ncols, data }
    }

    pub(crate) fn from(nrows: usize, ncols: usize, data: HashMap<usize, Vec<(usize, T)>>) -> Self {
        Matrix { nrows, ncols, data }
    }

    pub(crate) fn data(&self) -> &HashMap<usize, Vec<(usize, T)>> {
        &self.data
    }

    pub(crate) fn ncols(&self) -> usize {
        self.ncols
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding(&self, coding: &Vec<ExTensor>) -> Matrix<ExTensor> {
        let n = self.nrows;
        let mut data = HashMap::with_capacity(self.nrows * self.ncols);

        for (from, v) in self.data().iter() {
            let v: Vec<_> = v
                .into_iter()
                .map(|(to, _)| (*to, coding[*from].clone()))
                .collect();
            data.insert(*from, v);
        }

        Matrix::from(n, n, data)
    }
}

impl<T> std::ops::Mul<Vec<T>> for &Matrix<T>
where
    T: Zero + Clone + std::ops::Mul<Output = T>,
{
    type Output = Vec<T>;

    fn mul(self, other: Vec<T>) -> Vec<T> {
        assert_eq!(
            self.ncols,
            other.len(),
            "dimensions of vector and matrix do not match"
        );

        let mut res = vec![T::zero(); self.nrows];

        for (x, v) in self.data.iter() {
            let val = v.iter().fold(T::zero(), |acc, (y, val)| {
                acc + val.clone() * other[*y].clone()
            });
            res[*x] = val;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;

    use crate::matrix::sparse_hash::Matrix;
    use crate::utils;
    use num_traits::identities::Zero;

    #[test]
    fn mat_vec_mul() {
        let m = Matrix::new(2, 2, vec![1, 2, 0, 1]);
        let v = vec![1, 1];
        let r = &m * v;
        assert_eq!(r, vec![3, 1], "simple Matrix Vector multiplication");
    }

    #[test]
    fn mat_vec_mul_2() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let v = vec![1, 1, 1];
        let r = &m * v;
        assert_eq!(r, vec![6, 15], "simple Matrix Vector multiplication");
    }

    #[test]
    fn mat_vec_mul_3() {
        let m = Matrix::new(
            2,
            2,
            vec![
                ExTensor::new(&[1], &[vec![1]]),
                ExTensor::new(&[2], &[vec![2]]),
                ExTensor::new(&[3], &[vec![5]]),
                ExTensor::new(&[4], &[vec![6]]),
            ],
        );
        let v = vec![
            ExTensor::new(&[5], &[vec![3]]),
            ExTensor::new(&[6], &[vec![4]]),
        ];
        let res = vec![
            ExTensor::new(&[5, 12], &[vec![1, 3], vec![2, 4]]),
            ExTensor::new(&[-15, -24], &[vec![3, 5], vec![4, 6]]),
        ];
        assert_eq!(&m * v, res);
    }

    #[test]
    fn coding() {
        let k = 2;
        let n = 2;
        let coding = utils::create_vandermonde(n, k);
        let m: Matrix<u8> = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let n = m.add_coding(&coding);
        let expect = Matrix::new(
            2,
            2,
            vec![
                coding[0].clone(),
                coding[0].clone(),
                ExTensor::zero(),
                coding[1].clone(),
            ],
        );

        assert_eq!(n.data(), expect.data(), "add coding should work");
    }

    #[test]
    fn coding_2() {
        let k = 3;
        let n = 3;
        let coding = utils::create_vandermonde(n, k);
        let m: Matrix<u8> = Matrix::new(3, 3, vec![0, 1, 0, 1, 0, 1, 0, 1, 0]);
        let n = m.add_coding(&coding);
        let expect = Matrix::new(
            3,
            3,
            vec![
                ExTensor::zero(),
                coding[0].clone(),
                ExTensor::zero(),
                coding[1].clone(),
                ExTensor::zero(),
                coding[1].clone(),
                ExTensor::zero(),
                coding[2].clone(),
                ExTensor::zero(),
            ],
        );

        assert_eq!(n.data(), expect.data(), "add coding should work");
    }
}
