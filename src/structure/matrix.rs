use num_traits::identities::{One, Zero};
use std::collections::HashMap;

#[derive(Debug)]
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
}

impl<T> std::ops::Mul<Vec<T>> for Matrix<T>
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

        for (x, v) in self.data.into_iter() {
            let val = v
                .into_iter()
                .fold(T::zero(), |acc, (y, val)| acc + val * other[y].clone());
            res[x] = val;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::{extensor::ExTensor, matrix::Matrix};

    #[test]
    fn mat_vec_mul() {
        let m = Matrix::new(2, 2, vec![1, 2, 0, 1]);
        let v = vec![1, 1];
        let r = m * v;
        assert_eq!(r, vec![3, 1], "simple Matrix Vector multiplication");
    }

    /*
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
        let a = Matrix::new(2, 2, vec![0.0, 1.0, 2.0, 3.0]);
        let b = Matrix::new(2, 2, vec![3.0, 2.0, 1.0, 0.0]);
        let c = &a * &b;
        let expect = Matrix::new(2, 2, vec![1.0, 0.0, 9.0, 4.0]);
        assert_eq!(c.nrows, 2, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect, "square matrix multiplication");
    }

    #[test]
    fn mul_non_square() {
        let a = Matrix::new(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let c = &a * &b;
        let expect = Matrix::new(4, 2, vec![22, 28, 49, 64, 76, 100, 103, 136]);
        assert_eq!(c.nrows, 4, "rows of product match");
        assert_eq!(c.ncols, 2, "columns of product match");
        assert_eq!(c, expect, "non square matrix multiplication");
    }

    #[test]
    fn extensor_mat() {
        let v = vec![
            crate::extensor!([1.0], [[1]]),
            crate::extensor!([2.0], [[1]]),
            crate::extensor!([1.0], [[2]]),
            crate::extensor!([2.0], [[2]]),
        ];
        let t = Matrix::new(2, 2, v);
        let w = vec![
            crate::extensor!([2.0], [[2]]),
            crate::extensor!([1.0], [[2]]),
            crate::extensor!([1.0], [[1]]),
            crate::extensor!([2.0], [[1]]),
        ];
        let d = Matrix::new(2, 2, w);
        let prod = &t * &d;
        let r = vec![
            crate::extensor!([2.0], [[1, 2]]),
            crate::extensor!([1.0], [[1, 2]]),
            crate::extensor!([-2.0], [[1, 2]]),
            crate::extensor!([-4.0], [[1, 2]]),
        ];
        let expect = Matrix::new(2, 2, r);
        assert_eq!(
            prod, expect,
            "matrix multiplication with extensor components"
        );
    }

    #[test]
    fn mat_power() {
        let power = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).power(2);
        let expect = Matrix::new(3, 3, vec![30, 36, 42, 66, 81, 96, 102, 126, 150]);
        assert_eq!(power, expect, "3x3 matrix to the second power");
    }

    #[test]
    fn mat_power_big() {
        let power: Matrix<u128> = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).power(11);
        let expect: Matrix<u128> = Matrix::new(
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
            crate::extensor!([1.0], [[1]]),
            crate::extensor!([2.0], [[1]]),
            crate::extensor!([1.0], [[2]]),
            crate::extensor!([2.0], [[2]]),
        ];
        let power = Matrix::new(2, 2, v).power(2);
        let r = vec![
            crate::extensor!([2.0], [[1, 2]]),
            crate::extensor!([4.0], [[1, 2]]),
            crate::extensor!([-1.0], [[1, 2]]),
            crate::extensor!([-2.0], [[1, 2]]),
        ];
        let expect = Matrix::new(2, 2, r);
        assert_eq!(power, expect, "2x2 extensor matrix to the second power");
    }
    */
}
