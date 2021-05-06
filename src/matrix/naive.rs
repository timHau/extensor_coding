#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use num_traits::identities::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Matrix<T> {
    nrows: usize,
    ncols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone + One + Zero,
{
    pub(crate) fn new(nrows: usize, ncols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            nrows * ncols,
            "dimensons of values does not match"
        );

        Matrix { nrows, ncols, data }
    }

    pub(crate) fn data(&self) -> &Vec<T> {
        &self.data
    }

    #[allow(dead_code)]
    pub(crate) fn ncols(&self) -> usize {
        self.ncols
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding(&self, coding: &Vec<ExTensor>) -> Matrix<ExTensor> {
        let num_elems = self.nrows * self.ncols;
        let mut data = Vec::with_capacity(num_elems);
        data.reserve(num_elems);

        for (i, v) in self.data.iter().enumerate() {
            let row_index = i / self.ncols;
            if *v == 1 {
                data.push(coding[row_index].clone());
            } else {
                data.push(ExTensor::zero());
            }
        }

        Matrix {
            nrows: self.nrows,
            ncols: self.ncols,
            data,
        }
    }
}

impl<T> std::ops::Mul<Vec<T>> for &Matrix<T>
where
    T: Zero + Clone + std::ops::Mul<Output = T>,
{
    type Output = Vec<T>;

    fn mul(self, rhs: Vec<T>) -> Vec<T> {
        assert_eq!(
            self.ncols,
            rhs.len(),
            "dimensions of vector and matrix do not match"
        );

        let mut res = vec![T::zero(); self.nrows];

        for i in 0..self.nrows {
            let mut v = T::zero();
            for j in 0..self.ncols {
                v = v + self.data[i * self.ncols + j].clone() * rhs[j].clone();
            }
            res[i] = v;
        }

        res
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        self.data.get(index.0 * self.ncols + index.1).unwrap()
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.data[index.0 * self.ncols + index.1]
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;

    #[cfg(feature = "matrix_naive")]
    use crate::matrix::naive::Matrix;
    #[cfg(feature = "matrix_naive_parallel")]
    use crate::matrix::naive_parallel::Matrix;
    #[cfg(feature = "matrix_sparse_hash")]
    use crate::matrix::sparse_hash::Matrix;
    #[cfg(feature = "matrix_sparse_triples")]
    use crate::matrix::sparse_triples::Matrix;

    use crate::utils;
    use num_traits::identities::Zero;

    #[test]
    fn index() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        assert_eq!(m[(0, 0)], 1, "index (0, 0)");
        assert_eq!(m[(0, 1)], 2, "index (0, 1)");
        assert_eq!(m[(1, 0)], 3, "index (1, 0)");
        assert_eq!(m[(1, 1)], 4, "index (1, 1)");
    }

    #[test]
    fn index_mut() {
        let mut m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        m[(0, 0)] = 9;
        m[(0, 1)] = 8;
        m[(1, 0)] = 7;
        m[(1, 1)] = 6;
        assert_eq!(m[(0, 0)], 9, "mut index (0, 0)");
        assert_eq!(m[(0, 1)], 8, "mut index (0, 1)");
        assert_eq!(m[(1, 0)], 7, "mut index (1, 0)");
        assert_eq!(m[(1, 1)], 6, "mut index (1, 1)");
    }

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
