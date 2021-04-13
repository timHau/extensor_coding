use crate::structure::extensor::ExTensor;
use num_traits::identities::{One, Zero};
use std::{
    cmp::PartialEq,
    ops::{Add, Index, IndexMut, Mul, Sub},
    thread,
};

#[derive(Debug)]
pub(crate) struct MatrixSlice<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> MatrixSlice<T>
where
    T: Clone,
{
    fn new(data: &Vec<T>) -> Self {
        MatrixSlice {
            data: data.to_vec(),
            index: 0,
        }
    }
}

/// multiply two matrix slices.
/// (x_1 ... x_n)* (y_1 ... y_n)^T
impl<T> Mul for MatrixSlice<T>
where
    T: Default + Mul<Output = T> + Add<Output = T>,
{
    type Output = (usize, usize, T);
    fn mul(self, other: MatrixSlice<T>) -> (usize, usize, T) {
        let mut res = T::default();
        for (a, b) in self.data.into_iter().zip(other.data.into_iter()) {
            res = res + (a * b);
        }
        (self.index, other.index, res)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Matrix<T> {
    data: Vec<T>,
    nrows: usize,
    ncols: usize,
}

/// # Matrix
///
/// Implementation of a matrix, which is just a flat Vec
impl<T> Matrix<T>
where
    T: Default
        + One
        + Zero
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Send
        + 'static,
{
    /// ## new
    ///
    /// create a nrows x ncols matrix from the values inside vec
    pub(crate) fn new(nrows: usize, ncols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            nrows * ncols,
            data.len(),
            "dimensions of vec does not match"
        );
        Matrix { data, nrows, ncols }
    }

    /// ## zeros
    ///
    /// return the nrows x ncols matrix with all zeros
    pub(crate) fn zeros(nrows: usize, ncols: usize) -> Self {
        let mut data = Vec::with_capacity(nrows * ncols);
        for _ in 0..(nrows * ncols) {
            data.push(T::default());
        }
        Matrix { data, nrows, ncols }
    }

    /// ## ncols
    ///
    /// return the number of cols
    pub(crate) fn ncols(&self) -> usize {
        self.ncols
    }

    /// ## row
    ///
    /// return the row at index `i`
    fn row(&self, i: usize) -> MatrixSlice<T> {
        let index = i * self.ncols;
        let mut data = Vec::with_capacity(self.ncols);
        for j in index..(index + self.ncols) {
            data.push(self.data[j].clone());
        }
        MatrixSlice { data, index: i }
    }

    /// ## col
    ///
    /// return the column at index `i`
    fn col(&self, i: usize) -> MatrixSlice<T> {
        let index = i % self.nrows;
        let mut data = Vec::with_capacity(self.nrows);
        for j in (index..self.nrows * self.ncols).step_by(self.ncols) {
            data.push(self.data[j].clone());
        }
        MatrixSlice { data, index: i }
    }

    /// ## data
    ///
    /// return the components
    pub(crate) fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl Matrix<u8> {
    pub(crate) fn add_coding<F>(&self, coding: &F) -> Matrix<ExTensor>
    where
        F: Fn(usize) -> ExTensor,
    {
        let n = self.nrows;
        let mut data = Vec::with_capacity(n * n);

        for (i, v) in self.data.iter().enumerate() {
            if *v == 1 {
                let val = coding(i / n);
                data.push(val);
            } else {
                data.push(ExTensor::zero());
            }
        }

        Matrix::new(n, n, data)
    }
}

impl<T> Mul<Vec<T>> for &Matrix<T>
where
    T: Default
        + One
        + Zero
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Send
        + 'static,
{
    type Output = Vec<T>;
    fn mul(self, other: Vec<T>) -> Vec<T> {
        assert_eq!(self.ncols, other.len(), "dimensions of matrices dont match");
        let mut res = vec![T::zero(); self.nrows];

        let mut handles = vec![];
        for i in 0..self.nrows {
            for j in 0..other.len() {
                let row = self.row(i);
                let col = MatrixSlice::new(&other);
                let handle = thread::spawn(move || row * col);
                handles.push(handle);
            }
        }

        for handle in handles {
            let (i, _j, v) = handle.join().unwrap();
            res[i] = v
        }

        res
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        self.data.get(index.0 * self.ncols + index.1).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.data[index.0 * self.ncols + index.1]
    }
}

impl<T: PartialEq> PartialEq<Matrix<T>> for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::matrix_naive::Matrix;
    use crate::utils;

    #[test]
    fn zero() {
        let z: Matrix<u8> = Matrix::zeros(2, 2);
        let res: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(z.data, res, "zero 2x2 matrix works");
        assert_eq!(2 as usize, z.nrows, "zero matrix dimensions match");
        assert_eq!(2 as usize, z.ncols, "zero matrix dimensions match");
    }

    #[test]
    fn mul() {
        let m = Matrix::new(2, 2, vec![1, 2, 0, 1]);
        let v = vec![1, 1];
        let r = &m * v;
        assert_eq!(r, vec![3, 1], "simple Matrix Vector multiplication");
    }

    #[test]
    fn coding() {
        let k = 2;
        let (f_vert, _) = utils::create_vandermonde(k);
        let m: Matrix<u8> = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let n = m.add_coding(&f_vert);

        for ext in n.data() {
            println!("{}", ext);
        }
    }
}
