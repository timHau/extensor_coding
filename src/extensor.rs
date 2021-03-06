use std::{ops, fmt};
use indexmap::{IndexMap}; // Map with insertion ordering
use std::ops::Index;

#[derive(Debug)]
struct ExTensor {
    data: IndexMap<Vec<i32>, f64>,  // basis : coeff
}

impl ExTensor {
    /// create an new Extensor that does not need to be "simple"
    /// meaning sets with cardinality > 1 are supported
    /// example:
    /// ``` ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]) // 3e_{1,3} - 7e_{3} ```
    fn new(coeffs: &[f64], basis: &[&[i32]]) -> ExTensor {
        assert_eq!(coeffs.len(), basis.len(), "coeffs and basis must be of same length");
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
}

impl ops::Add<ExTensor> for ExTensor {
    type Output = ExTensor;

    fn add(self, rhs: ExTensor) -> ExTensor {
        let mut data = IndexMap::new();

        let joined_data = self.data.iter().chain(rhs.data.iter());
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

    fn mul(self, ex: ExTensor) -> ExTensor {
        let mut data = IndexMap::new();
        for val in ex.data {
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
        let x = ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]);
        let y = ExTensor::new(&[1.0, 2.0], &[&[1], &[3]]);
        let sum = x + y;
        let res = ExTensor::new(&[3.0, -5.0, 1.0], &[&[1, 3], &[3], &[1]]);
        assert_eq!(sum.data, res.data);
    }

    #[test]
    fn scalar_mul() {
        let x_1 = ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]) * 2.0;
        let x_2= 2.0 * ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]);
        let res = ExTensor::new(&[6.0, 4.0], &[&[1, 2], &[3, 4]]);
        assert_eq!(x_1.data, res.data);
        assert_eq!(x_2.data, res.data);
        assert_eq!(x_1.data, x_2.data);
    }
}