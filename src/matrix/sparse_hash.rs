#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

use num_traits::identities::{One, Zero};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    pub nrows: usize,
    pub ncols: usize,
    pub data: HashMap<usize, Vec<(usize, T)>>,
}

/// # Matrix
///
/// Create a Matrix that stores values of type `T`. `T` only needs to
/// be clonable and must have a zero and a one element.
/// Implements a sparse Matrix based on a HashMap.
/// Each row that has non zero elements corresponds to a vec in the HashMap
/// The row Index is the key. The non zero value is stored in the Format (col index, value) as a value.
///
/// Example:
///
/// ```no code
/// | 1 0 0 1 |                  0: vec![ (0, 1), (3, 1) ]
/// | 0 0 1 0 |     ------>      1: vec![ (2, 1) ]
/// | 0 0 0 1 |                  2: vec![ (3, 1) ]
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

        let mut data = HashMap::new();

        for (i, val) in values.into_iter().enumerate() {
            if !val.is_zero() {
                let row_index = i / ncols;
                let col_index = i % ncols;

                let row_vals = data.entry(row_index).or_insert(Vec::new());
                row_vals.push((col_index, val));
            }
        }

        Matrix { nrows, ncols, data }
    }

    pub(crate) fn from(nrows: usize, ncols: usize, data: HashMap<usize, Vec<(usize, T)>>) -> Self {
        Matrix { nrows, ncols, data }
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding(&self, coding: &Vec<ExTensor>) -> Matrix<ExTensor> {
        let mut data = HashMap::with_capacity(self.nrows * self.ncols);

        for (from, v) in self.data.iter() {
            let v: Vec<_> = v
                .into_iter()
                .map(|(to, _)| (*to, coding[*from].clone()))
                .collect();
            data.insert(*from, v);
        }

        Matrix::from(self.nrows, self.ncols, data)
    }

    pub(crate) fn neighbors_of(&self, i: usize) -> Vec<usize> {
        let row = self.data.get(&i);

        match row {
            None => vec![],
            Some(row) => row.iter().map(|(i, _val)| *i).collect(),
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

        for (x, v) in self.data.iter() {
            let val = v.iter().fold(T::zero(), |acc, (y, val)| {
                acc + val.clone() * rhs[*y].clone()
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
    use std::collections::HashMap;

    #[test]
    fn create() {
        let m = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let mut expect = HashMap::new();
        expect.insert(0, vec![(0, 1), (1, 1)]);
        expect.insert(1, vec![(1, 1)]);
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
        let mut expect = HashMap::new();
        expect.insert(0, vec![(1, 1), (2, 2), (3, 3), (4, 4)]);
        expect.insert(1, vec![(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);
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
        let mut expect = HashMap::new();
        expect.insert(0, vec![(0, 1), (3, 1)]);
        expect.insert(1, vec![(2, 1)]);
        expect.insert(2, vec![(3, 1)]);
        assert_eq!(m.data, expect, "Matrix should be created correctly");
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
    fn mat_vec_mul_4() {
        let m = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v = vec![1, 2, 3];
        let r = &m * v;
        assert_eq!(r, vec![14, 32, 50], "simple Matrix Vector multiplication");
    }

    #[test]
    fn mat_vec_mul_5() {
        let m = Matrix::new(
            2,
            2,
            vec![
                ExTensor::new(&[1], &[vec![1, 2]]),
                ExTensor::new(&[2], &[vec![3, 4]]),
                ExTensor::new(&[3], &[vec![1, 2]]),
                ExTensor::new(&[4], &[vec![3, 4]]),
            ],
        );
        let v = vec![
            ExTensor::new(&[9], &[vec![1]]),
            ExTensor::new(&[8], &[vec![3]]),
        ];
        let r = &m * v;
        assert_eq!(r.len(), 2, "dimensions match");
        assert_eq!(r[0].is_zero(), true, "first entry vanishes");
        assert_eq!(r[1].is_zero(), true, "second entry vanishes");
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
