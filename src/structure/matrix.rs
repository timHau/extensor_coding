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

    pub(crate) fn from(nrows: usize, ncols: usize, data: HashMap<usize, Vec<(usize, T)>>) -> Self {
        Matrix { nrows, ncols, data }
    }

    pub(crate) fn nrows(&self) -> usize {
        self.nrows
    }

    pub(crate) fn data(&self) -> &HashMap<usize, Vec<(usize, T)>> {
        &self.data
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

        for (x, v) in self.data.iter() {
            let val = v
                .iter()
                .fold(T::zero(), |acc, (y, val)| {
                    acc + val.clone() * other[*y].clone()
                });
            res[*x] = val;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::matrix::Matrix;

    #[test]
    fn mat_vec_mul() {
        let m = Matrix::new(2, 2, vec![1, 2, 0, 1]);
        let v = vec![1, 1];
        let r = &m * v;
        assert_eq!(r, vec![3, 1], "simple Matrix Vector multiplication");
    }
}
