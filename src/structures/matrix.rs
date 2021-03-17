use std::cmp::PartialEq;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

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
    T: std::fmt::Debug + Default + Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, b: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.ncols, b.nrows, "dimensions of matrices dont match");
        let mut data = Vec::new();

        for i in 0..self.nrows {
            for j in 0..b.ncols {
                let mut c = T::default();
                for k in 0..self.ncols {
                    let r = self[(i, k)].clone() * b[(k, j)].clone();
                    c = c + r;
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
    use crate::structures::matrix::Matrix;
    use crate::structures::extensor::ExTensor;

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
        assert_eq!(c, expect, "square matrix multiplication");
    }

    #[test]
    fn test_mul_non_square() {
        let a = Matrix::from_vec(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let b = Matrix::from_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let c = &a * &b;
        let expect = Matrix::from_vec(4, 2, vec![22, 28, 49, 64, 76, 100, 103, 136]);
        assert_eq!(c.nrows, 4, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect, "non square matrix multiplication");
    }

    #[test]
    fn test_extensor_mat() {
        let v = vec![ExTensor::simple(1.0, 1), ExTensor::simple(2.0, 1), ExTensor::simple(1.0, 2), ExTensor::simple(2.0, 2)];
        let t = Matrix::from_vec(2, 2, v);
        let w = vec![ExTensor::simple(2.0, 2), ExTensor::simple(1.0, 2), ExTensor::simple(1.0, 1), ExTensor::simple(2.0, 1)];
        let d = Matrix::from_vec(2, 2, w);
        let prod = &t * &d;
        let r = vec![ExTensor::new(&[2.0], &[&[1, 2]]), ExTensor::new(&[1.0], &[&[1, 2]]), ExTensor::new(&[-2.0], &[&[1, 2]]), ExTensor::new(&[-4.0], &[&[1, 2]])];
        let expect = Matrix::from_vec(2, 2, r);
        assert_eq!(prod, expect, "matrix multiplication with extensor components");
    }
}
