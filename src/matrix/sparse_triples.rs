#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use num_traits::identities::{One, Zero};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub(crate) struct Matrix<T> {
    nrows: usize,
    ncols: usize,
    data: Vec<(usize, usize, T)>,
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

        let num_elems = nrows * ncols;
        let mut data = Vec::with_capacity(num_elems);
        data.reserve(num_elems);

        for (i, val) in values.into_iter().enumerate() {
            if !val.is_zero() {
                let row_index = i / ncols;
                let col_index = i % nrows;

                data.push((row_index, col_index, val));
            }
        }

        Matrix { nrows, ncols, data }
    }

    pub(crate) fn data(&self) -> &Vec<(usize, usize, T)> {
        &self.data
    }

    #[allow(dead_code)]
    pub(crate) fn ncols(&self) -> usize {
        self.ncols
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding<F>(&self, coding: &F) -> Matrix<ExTensor>
    where
        F: Fn(usize) -> ExTensor,
    {
        let n = self.nrows;
        let num_elems = self.nrows * self.ncols;
        let mut data = Vec::with_capacity(num_elems);
        data.reserve(num_elems);

        for (i, (x, y, _v)) in self.data.iter().enumerate() {
            let val = coding((i / n) + 1);
            data.push((*x, *y, val));
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

    fn mul(self, other: Vec<T>) -> Vec<T> {
        assert_eq!(
            self.ncols,
            other.len(),
            "dimensions of vector and matrix do not match"
        );

        let mut res = vec![T::zero(); self.nrows];

        for (x, y, v) in self.data.iter() {
            res[*x] = res[*x].clone() + v.clone() * other[*y].clone();
        }

        res
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        let (i, j) = index;

        let res = self
            .data
            .iter()
            .filter(|(x, y, _v)| *x == i && *y == j)
            .collect::<Vec<_>>()[0];
        let res = Some(&res.2);

        res.unwrap()
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (i, j) = index;

        let mut index = 0;
        for (k, (x, y, _v)) in self.data.iter().enumerate() {
            if *x == i && *y == j {
                index = k;
            }
        }

        let (_x, _y, v) = self.data[index].borrow_mut();
        v
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;

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
        let (f_vert, _) = utils::create_vandermonde(k);
        let m: Matrix<u8> = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let n = m.add_coding(&f_vert);
        let expect = Matrix::new(
            2,
            2,
            vec![f_vert(1), f_vert(1), ExTensor::zero(), f_vert(2)],
        );

        assert_eq!(n.data(), expect.data(), "add coding should work");
    }
}
