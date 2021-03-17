use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Index, Mul, Not, Sub};

#[derive(Debug)]
pub(crate) struct Matrix<T> {
    data: Vec<T>,
    nrows: usize,
    ncols: usize,
}

impl<T> Matrix<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// create a nrows x ncols matrix from the values inside vec
    pub(crate) fn from_vec(nrows: usize, ncols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            nrows * ncols,
            data.len(),
            "dimensions of vec does not match"
        );
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
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Copy + Default + Mul<Output = T> + AddAssign,
{
    type Output = Matrix<T>;

    fn mul(self, b: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.ncols, b.ncols, "dimensions of matrices dont match");
        let mut data = Vec::new();

        for i in 0..self.ncols {
            for j in 0..b.nrows {
                let mut c = T::default();
                for k in 0..self.ncols {
                    c += self[(i, k)] * b[(k, j)];
                }
                data.push(c);
            }
        }

        Matrix {
            data,
            nrows: self.nrows,
            ncols: b.ncols,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        self.data.get(index.0 * self.nrows + index.1).unwrap()
    }
}

impl<T: PartialEq> PartialEq<Matrix<T>> for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::matrix::Matrix;

    #[test]
    fn test_zero() {
        let z: Matrix<u8> = Matrix::zeros(2, 2);
        let res: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(z.data, res, "zero 2x2 matrix works");
        assert_eq!(2 as usize, z.nrows, "zero matrix dimensions match");
        assert_eq!(2 as usize, z.ncols, "zero matrix dimensions match");
    }

    #[test]
    fn test_mul() {
        let a = Matrix::from_vec(2, 2, vec![0.0, 1.0, 2.0, 3.0]);
        let b = Matrix::from_vec(2, 2, vec![3.0, 2.0, 1.0, 0.0]);
        let c = &a * &b;
        let expect = Matrix::from_vec(2, 2, vec![1.0, 0.0, 9.0, 4.0]);
        assert_eq!(c.nrows, 2, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect);
    }
}
