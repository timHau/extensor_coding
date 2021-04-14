use crate::extensor::ExTensor;
use num_traits::identities::{One, Zero};

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
            let val = coding(i / n);
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

#[cfg(test)]
mod tests {
    use crate::matrix::sparse_triples::Matrix;
    use crate::utils;

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

        println!("n");
        for (x, y, ext) in n.data() {
            println!("{}", ext);
        }
    }
}
