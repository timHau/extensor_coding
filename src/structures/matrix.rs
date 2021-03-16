use std::cmp::max;
use std::convert::Into;
use std::ops::{Add, Mul, Sub};
use std::process::Output;

#[derive(Debug)]
pub(crate) struct Matrix<T> {
    data: Vec<T>,
    nrows: usize,
    ncols: usize,
}

impl<T> Matrix<T>
where
    T: Clone + Default + Sub<Output = T> + Add<Output = T>,
    for<'a> &'a T: Mul<Output = T>,
{
    /// create a nrows x ncols matrix from the values inside vec
    pub(crate) fn from_vec(nrows: usize, ncols: usize, vec: &Vec<T>) -> Self {
        assert_eq!(nrows * ncols, vec.len(), "dimensions of vec does not match");
        let data = vec.to_vec();
        Matrix { data, nrows, ncols }
    }

    /// return the nrows x ncols matrix with all zeros
    pub(crate) fn zeros(nrows: usize, ncols: usize) -> Self {
        let mut data = Vec::new();
        for _ in 0..(nrows * ncols) {
            data.push(T::default());
        }
        Matrix { data, nrows, ncols }
    }

    pub(crate) fn nrows(&self) -> usize {
        self.nrows
    }

    pub(crate) fn ncols(&self) -> usize {
        self.nrows
    }

    /// get the value at position i, j
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.data.get(i * self.nrows + j)
    }

    /// compute the value of the determinante
    /// for a 2x2 or 3x3 matrix use sarrus rule
    /// for bigger matrices use LU decomposition
    pub(crate) fn determinant(&self) -> T {
        assert_eq!(
            self.nrows, self.ncols,
            "can only compute determinante on square matrices"
        );
        match self.nrows {
            0 => T::default(),
            1 => self.get(0, 0).unwrap().clone(),
            2 => {
                let m11 = self.get(0, 0).unwrap();
                let m12 = self.get(0, 1).unwrap();
                let m21 = self.get(1, 0).unwrap();
                let m22 = self.get(1, 1).unwrap();

                m11 * m22 - m21 * m12
            }
            3 => {
                let m11 = self.get(0, 0).unwrap();
                let m12 = self.get(0, 1).unwrap();
                let m13 = self.get(0, 2).unwrap();

                let m21 = self.get(1, 0).unwrap();
                let m22 = self.get(1, 1).unwrap();
                let m23 = self.get(1, 2).unwrap();

                let m31 = self.get(2, 0).unwrap();
                let m32 = self.get(2, 1).unwrap();
                let m33 = self.get(2, 2).unwrap();

                &(m11 * m22) * m33 + &(m12 * m23) * m31 + &(m13 * m21) * m32
                    - &(m31 * m22) * m13
                    - &(m32 * m23) * m11
                    - &(m33 * m21) * m12
            }
            _ => T::default(),
        }
    }
}

/// implement matrix multiplication
/// using a parallel algorithm, might be improved by using Strassen Algorithm or similar ones
impl<T> std::ops::Mul<&Matrix<T>> for &Matrix<T>
where
    T: Clone + std::ops::Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, b: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.ncols, b.ncols, "dimensions of matrices dont match");
        // split matrices into two submatrices
        // this will also work on matricies that dont have dimension 2^k x 2^k
        // Assume self has dimensions n x m, b has dimensions m x p
        // define a thresold, if max(n, m, p) < threshold => use other algorithm
        let thresshold = &(20 as usize);
        if max(max(&self.nrows, &self.ncols), &b.ncols) < thresshold {
            println!("TODO");
        }

        Matrix {
            data: vec![],
            ncols: 0,
            nrows: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::matrix::Matrix;

    #[test]
    fn test_zero() {
        let z = Matrix::<u8>::zeros(2, 2);
        let res: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(z.data, res, "zero 2x2 matrix works");
        assert_eq!(2 as usize, z.nrows, "zero matrix dimensions match");
        assert_eq!(2 as usize, z.ncols, "zero matrix dimensions match");
    }
}
