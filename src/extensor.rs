use indexmap::IndexMap;
use permutation::permutation;
use std::iter::FromIterator;
use std::{fmt, ops};

#[derive(Debug)]
struct ExTensor {
    data: IndexMap<Vec<i32>, f64>, // basis : coeff
}

impl ExTensor {
    /// create an new Extensor that does not need to be "simple"
    /// meaning sets with cardinality > 1 are supported
    /// example:
    /// ``` ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]) // 3e_{1,3} - 7e_{3} ```
    fn new(coeffs: &[f64], basis: &[&[i32]]) -> ExTensor {
        assert_eq!(
            coeffs.len(),
            basis.len(),
            "coeffs and basis must be of same length"
        );
        let mut data = IndexMap::new();
        for i in 0..basis.len() {
            data.insert(basis[i].to_vec(), coeffs[i]);
        }
        ExTensor { data }
    }

    /// construct a simple exterior tensor e.g. only using a single basis set
    /// example:
    /// ``` Extensor::simple(9.0, 3) // 9e_{3} ```
    fn simple(coeff: f64, basis: i32) -> ExTensor {
        let mut data = IndexMap::new();
        data.insert(vec![basis], coeff);
        ExTensor { data }
    }

    /// get sign of permutation that brings the basis at 'basis_index' into increasing order
    /// output ∈ {-1, 1}
    fn get_sign(&self, basis_index: usize) -> i32 {
        // from here: https://math.stackexchange.com/questions/65923/how-does-one-compute-the-sign-of-a-permutation
        let v = self.data.get_index(basis_index).unwrap().0; // get the basis at basis_index
        let perm = permutation::sort(&v[..]); // get permutation that would sort that basis
        let p = perm.apply_slice(Vec::from_iter(0..v.len()));

        let mut visited = vec![false; v.len()]; // mark all visited
        let mut sign = 1;
        for k in 0..v.len() {
            if !visited[k] {
                let mut next = k;
                let mut l = 0;
                while !visited[next] {
                    l += 1;
                    visited[next] = true;
                    next = p[next];
                }
                if l % 2 == 0 {
                    sign = -sign;
                }
            }
        }

        sign
    }
}

impl ops::Add<ExTensor> for ExTensor {
    type Output = ExTensor;

    fn add(self, rhs: ExTensor) -> ExTensor {
        let mut data = IndexMap::new();

        let joined_data: Vec<_> = self.data.iter().chain(rhs.data.iter()).collect();
        for val in joined_data {
            let basis = val.0.to_vec();
            let coeff = *val.1;
            if data.contains_key(&basis) {
                let c = data.get(&basis).unwrap();
                data.insert(basis, c + coeff);
            } else {
                data.insert(basis, coeff);
            }
        }

        ExTensor { data }
    }
}

impl ops::Mul<&ExTensor> for &ExTensor {
    type Output = ExTensor;

    fn mul(self, rhs: &ExTensor) -> ExTensor {
        let mut data = IndexMap::new();
        for d in self.data.iter() {
            data.insert(d.0.to_vec(), *d.1);
        }
        for d in rhs.data.iter() {
            if data.contains_key(d.0) {
                data.insert(d.0.to_vec(), data[d.0] * d.1);
            } else {
                data.insert(d.0.to_vec(), *d.1);
            }
        }

        ExTensor { data }
    }
}

impl ops::Mul<f64> for ExTensor {
    type Output = ExTensor;

    fn mul(self, c: f64) -> ExTensor {
        let mut data = IndexMap::new();
        for val in self.data {
            data.insert(val.0, val.1 * c);
        }
        ExTensor { data }
    }
}

impl ops::Mul<ExTensor> for f64 {
    type Output = ExTensor;

    fn mul(self, rhs: ExTensor) -> ExTensor {
        let mut data = IndexMap::new();
        for val in rhs.data {
            data.insert(val.0, self * val.1);
        }
        ExTensor { data }
    }
}

impl fmt::Display for ExTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for d in self.data.iter() {
            s += format!("{}e_{:?} ", d.1, d.0).as_str();
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod extensor_tests {
    use crate::extensor::ExTensor;
    use std::collections::HashMap;

    #[test]
    fn add() {
        let x_1 = ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]);
        let x_2 = ExTensor::new(&[1.0, 2.0], &[&[1], &[3]]);
        let sum = x_1 + x_2;
        let res = ExTensor::new(&[3.0, -5.0, 1.0], &[&[1, 3], &[3], &[1]]);
        assert_eq!(sum.data, res.data);
    }

    #[test]
    fn scalar_mul() {
        let x_1 = ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]) * 2.0;
        let x_2 = 2.0 * ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]);
        let res = ExTensor::new(&[6.0, 4.0], &[&[1, 2], &[3, 4]]);
        assert_eq!(x_1.data, res.data);
        assert_eq!(x_2.data, res.data);
        assert_eq!(x_1.data, x_2.data);
    }

    #[test]
    fn sign() {
        let x_1 = ExTensor::new(&[3.0], &[&[2, 1]]);
        assert_eq!(x_1.get_sign(0), -1);
        let x_2 = ExTensor::new(&[3.0], &[&[1, 2]]);
        assert_eq!(x_2.get_sign(0), 1);
        let x_3 = ExTensor::new(&[1.0, 1.0], &[&[2, 3, 4, 5, 6], &[2, 1]]);
        assert_eq!(x_3.get_sign(0), 1);
        assert_eq!(x_3.get_sign(1), -1);
    }

    #[test]
    fn mul() {
        let x_1 = &ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]);
        let x_2 = &ExTensor::new(&[1.0, 2.0], &[&[1], &[3]]);
        let prod_1 = x_1 * x_2;
        let res = ExTensor::new(&[3.0, -14.0, 1.0], &[&[1, 3], &[3], &[1]]);
        assert_eq!(prod_1.data, res.data);
        let prod_2 = x_2 * x_1;
        assert_eq!(prod_1.data, prod_2.data);

        let x_3 = &ExTensor::simple(2.0, 1);
        let x_4 = &ExTensor::simple(2.0, 1);
        let prod_3 = x_3 * x_4;
        let res = ExTensor::simple(4.0, 1);
        assert_eq!(prod_3.data, res.data);
    }
}
