use std::cmp::PartialEq;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug)]
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
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// ## from_vec
    ///
    /// create a nrows x ncols matrix from the values inside vec
    pub(crate) fn from_vec(nrows: usize, ncols: usize, data: Vec<T>) -> Self {
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

    pub(crate) fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// ## power
    ///
    /// naive implementation of a matrix power
    /// can be optimised by first diagonalizing and then taking the eigenvalues to a power
    pub(crate) fn power(&self, k: usize) -> Self {
        let mut b = Matrix::from_vec(self.nrows.clone(), self.ncols.clone(), self.data.clone());
        for i in 1..k {
            b = &b * &self;
        }
        b
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
    use crate::structure::extensor::ExTensor;
    use crate::structure::matrix::Matrix;

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
        let a = Matrix::from_vec(2, 2, vec![0.0, 1.0, 2.0, 3.0]);
        let b = Matrix::from_vec(2, 2, vec![3.0, 2.0, 1.0, 0.0]);
        let c = &a * &b;
        let expect = Matrix::from_vec(2, 2, vec![1.0, 0.0, 9.0, 4.0]);
        assert_eq!(c.nrows, 2, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect, "square matrix multiplication");
    }

    #[test]
    fn mul_non_square() {
        let a = Matrix::from_vec(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let b = Matrix::from_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let c = &a * &b;
        let expect = Matrix::from_vec(4, 2, vec![22, 28, 49, 64, 76, 100, 103, 136]);
        assert_eq!(c.nrows, 4, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect, "non square matrix multiplication");
    }

    #[test]
    fn extensor_mat() {
        let v = vec![
            ExTensor::simple(1.0, 1),
            ExTensor::simple(2.0, 1),
            ExTensor::simple(1.0, 2),
            ExTensor::simple(2.0, 2),
        ];
        let t = Matrix::from_vec(2, 2, v);
        let w = vec![
            ExTensor::simple(2.0, 2),
            ExTensor::simple(1.0, 2),
            ExTensor::simple(1.0, 1),
            ExTensor::simple(2.0, 1),
        ];
        let d = Matrix::from_vec(2, 2, w);
        let prod = &t * &d;
        let r = vec![
            ExTensor::new(&[2.0], &[&[1, 2]]),
            ExTensor::new(&[1.0], &[&[1, 2]]),
            ExTensor::new(&[-2.0], &[&[1, 2]]),
            ExTensor::new(&[-4.0], &[&[1, 2]]),
        ];
        let expect = Matrix::from_vec(2, 2, r);
        assert_eq!(
            prod, expect,
            "matrix multiplication with extensor components"
        );
    }

    #[test]
    fn mat_power() {
        let power = Matrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).power(2);
        let expect = Matrix::from_vec(3, 3, vec![30, 36, 42, 66, 81, 96, 102, 126, 150]);
        assert_eq!(power, expect, "3x3 matrix to the second power");
    }

    #[test]
    fn mat_power_big() {
        let power: Matrix<u128> = Matrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).power(11);
        let expect = Matrix::from_vec(
            3,
            3,
            vec![
                2135095631568,
                2623420941336,
                3111746251104,
                4835149302222,
                5941013482665,
                7046877663108,
                7535202972876,
                9258606023994,
                10982009075112,
            ],
        );
        assert_eq!(power, expect, "3x3 matrix to the 11th power");
    }

    #[test]
    fn mat_extensor_power() {
        let v = vec![
            ExTensor::simple(1.0, 1),
            ExTensor::simple(2.0, 1),
            ExTensor::simple(1.0, 2),
            ExTensor::simple(2.0, 2),
        ];
        let power = Matrix::from_vec(2, 2, v).power(2);
        let r = vec![
            ExTensor::new(&[2.0], &[&[1, 2]]),
            ExTensor::new(&[4.0], &[&[1, 2]]),
            ExTensor::new(&[-1.0], &[&[1, 2]]),
            ExTensor::new(&[-2.0], &[&[1, 2]]),
        ];
        let expect = Matrix::from_vec(2, 2, r);
        assert_eq!(power, expect, "2x2 extensor matrix to the second power");
    }
}
