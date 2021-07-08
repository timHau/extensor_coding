#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use num_traits::identities::{One, Zero};
use std::borrow::BorrowMut;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub nrows: usize,
    pub ncols: usize,
    pub data: Vec<(usize, usize, T)>,
}

/// # Matrix
///
/// Create a Matrix that stores values of type `T`. `T` only needs to
/// be clonable and must have a zero and a one element.
/// Implements a sparse Matrix based on triples.
/// The `(i, j)`-th Element with value `v` in the Matrix corresponds to the triple `(i, j, v)`.
///
/// Example:
///
/// ```no code
/// | 1 0 0 1 |                 vec![ (0, 0, 1), (0, 3, 1)
/// | 0 0 1 0 |     ------>           (1, 2, 1),
/// | 0 0 0 1 |                       (2, 3, 1) ]
/// ```
impl<T> Matrix<T>
where
    T: Clone + One + Zero,
{
    /// ## new
    ///
    /// Create a new Matrix.
    ///
    /// Arguments:
    ///
    /// `nrows`: number of rows
    /// `ncols`: number of columns
    /// `values`: Vec of values, size of Vec must be nrows*ncols
    ///
    pub fn new(nrows: usize, ncols: usize, values: Vec<T>) -> Self {
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
                let col_index = i % ncols;

                data.push((row_index, col_index, val));
            }
        }

        Matrix { nrows, ncols, data }
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding(&self, coding: &Vec<ExTensor>) -> Matrix<ExTensor> {
        let num_elems = self.nrows * self.ncols;
        let mut data = Vec::with_capacity(num_elems);
        data.reserve(num_elems);

        for (x, y, _v) in self.data.iter() {
            let val = coding[*x].clone();
            data.push((*x, *y, val));
        }

        Matrix {
            nrows: self.nrows,
            ncols: self.ncols,
            data,
        }
    }

    pub(crate) fn neighbors_of(&self, i: usize) -> Vec<usize> {
        self.data
            .iter()
            .filter(|(row, _col, _v)| *row == i)
            .map(|(_row, col, _val)| *col)
            .collect()
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

        for (x, y, v) in self.data.iter() {
            res[*x] = res[*x].clone() + rhs[*y].clone() * v.clone();
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
    fn create() {
        let m = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let expect = vec![(0, 0, 1), (0, 1, 1), (1, 1, 1)];
        assert_eq!(m.data, expect, "Matrix should be created correctly");
    }

    #[test]
    fn create_rect() {
        let m = Matrix::new(
            2,
            5,
            vec![
                0, 1, 2, 3, 4, // first row
                5, 6, 7, 8, 9, // second row
            ],
        );
        let expect = vec![
            (0, 1, 1),
            (0, 2, 2),
            (0, 3, 3),
            (0, 4, 4),
            (1, 0, 5),
            (1, 1, 6),
            (1, 2, 7),
            (1, 3, 8),
            (1, 4, 9),
        ];
        assert_eq!(m.data, expect, "Matrix should be created correctly");
    }

    #[test]
    fn create_simple() {
        let m = Matrix::new(
            3,
            4,
            vec![
                1, 0, 0, 1, //
                0, 0, 1, 0, //
                0, 0, 0, 1, //
            ],
        );
        let expect = vec![(0, 0, 1), (0, 3, 1), (1, 2, 1), (2, 3, 1)];
        assert_eq!(m.data, expect, "Matrix should be created correctly");
    }

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

        assert_eq!(n.data, expect.data, "add coding should work");
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

        assert_eq!(n.data, expect.data, "add coding should work");
    }

    #[test]
    fn neighbors() {
        let m: Matrix<u8> = Matrix::new(4, 4, vec![0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0]);
        let neighbors_0 = m.neighbors_of(0);
        let expect_0 = vec![1];
        let neighbors_1 = m.neighbors_of(1);
        let expect_1 = vec![0, 2];
        let neighbors_2 = m.neighbors_of(2);
        let expect_2 = vec![1, 2, 3];
        let neighbors_3 = m.neighbors_of(3);
        let expect_3 = vec![];

        assert_eq!(neighbors_0, expect_0);
        assert_eq!(neighbors_1, expect_1);
        assert_eq!(neighbors_2, expect_2);
        assert_eq!(neighbors_3, expect_3);
    }
}
