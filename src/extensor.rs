use indexmap::IndexMap;
use permutation::{permutation};
use std::iter::FromIterator;
use std::{fmt, ops, cmp};

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
        ExTensor { data }.sorted()
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

    fn sorted(&self) -> ExTensor {
        let mut data = IndexMap::new();

        for (i, d) in self.data.iter().enumerate() {
            let sign = self.get_sign(i) as f64;
            let mut basis_next = d.0.to_vec();
            basis_next.sort();
            if data.contains_key(&basis_next) {
                let coeff_old = data.get(&basis_next).unwrap();
                data.insert(basis_next, coeff_old + (d.1 * sign));
            } else {
                data.insert(basis_next, d.1 * sign);
            }
        }

        ExTensor { data }
    }
}

impl ops::Add<&ExTensor> for &ExTensor {
    type Output = ExTensor;

    fn add(self, rhs: &ExTensor) -> ExTensor {
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

        for val_lhs in self.data.iter() {
            let basis_lhs = val_lhs.0;
            let coeff_lhs = val_lhs.1;

            for val_rhs in rhs.data.iter() {
                let basis_rhs = val_rhs.0;
                let coeff_rhs = val_rhs.1;
                let basis_next: Vec<_> = [&basis_lhs[..], &basis_rhs[..]].concat();

                if super::utils::has_unique_elements(&basis_next) {
                    let coeff_next = coeff_rhs * coeff_lhs;
                    data.insert(basis_next, coeff_next);
                }
            }
        }

        ExTensor { data }.sorted()
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

impl cmp::PartialEq<ExTensor> for ExTensor {
    fn eq(&self, other: &ExTensor) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for ExTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if self.data.len() == 0 {
            return write!(f, "0");
        }

        let mut count_term = 0;
        for d in self.data.iter() {
            let base = String::new();
            if *d.1 < 0.0 {
                s += format!("({}) ", d.1).as_str();
            } else {
                s += format!("{} ", d.1).as_str();
            }

            let mut count_base = 0;
            for b in d.0.iter() {
                if count_base == d.0.len() - 1 {
                    if count_term == self.data.len() - 1 {
                        s += format!("e_{}", b).as_str();
                    } else {
                        s += format!("e_{}  +  ", b).as_str();
                    }
                } else {
                    s += format!("e_{} ∧ ", b).as_str();
                }
                count_base += 1;
            }

            count_term += 1;
        }


        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod extensor_tests {
    use crate::extensor::ExTensor;
    use std::collections::HashMap;
    use indexmap::map::IndexMap;

    #[test]
    fn add() {
        let x_1 = &ExTensor::new(&[3.0, -7.0], &[&[1, 3], &[3]]);
        let x_2 = &ExTensor::new(&[1.0, 2.0], &[&[1], &[3]]);
        let sum = x_1 + x_2;
        let res = ExTensor::new(&[3.0, -5.0, 1.0], &[&[1, 3], &[3], &[1]]);
        assert_eq!(sum, res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        assert_eq!(sum, sum_2, "exterior sum is commutative")
    }

    #[test]
    fn scalar_mul() {
        let x_1 = ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]) * 2.0;
        let x_2 = 2.0 * ExTensor::new(&[3.0, 2.0], &[&[1, 2], &[3, 4]]);
        let res = ExTensor::new(&[6.0, 4.0], &[&[1, 2], &[3, 4]]);
        assert_eq!(x_1, res, "scalar multiplication is right commutative");
        assert_eq!(x_2, res, "scalar multiplication is left commutative");
        assert_eq!(x_1, x_2, "scalar multiplication is commutative");
    }

    #[test]
    fn sign() {
        let x_1 = ExTensor::new(&[-3.0], &[&[2, 1]]);
        let x_2 = ExTensor::new(&[3.0], &[&[1, 2]]);
        assert_eq!(x_1, x_2, "exterior tensors are anti commutative");
    }

    #[test]
    fn mul() {
        let x_1 = &ExTensor::simple(1.0, 1);
        let prod_1 = x_1 * x_1;
        let zero_tensor = &ExTensor::new(&[], &[]);
        // assert_eq!(prod_1, zero_tensor, "x wedge x vanishes");

        // test anti-commutativity
        let x_3 = &ExTensor::simple(2.0, 1);
        let x_4 = &ExTensor::simple(4.0, 3);
        let prod_4 = x_3 * x_4;
        let res_1 = ExTensor::new(&[8.0], &[&[1, 3]]);
        let prod_5 = x_4 * x_3;
        let res_anti = ExTensor::new(&[-8.0], &[&[1, 3]]);
        assert_eq!(prod_4, res_1, "wedge product on simple extensors");
        assert_eq!(prod_5, res_anti, "wedge product on simple extensors is anti communative");

        let x_5 = &ExTensor::new(&[2.0, 3.0], &[&[1], &[2]]);
        let x_6 = &ExTensor::new(&[4.0, 5.0], &[&[1], &[2]]);
        let prod = &(x_5 * x_6);
        let det = na::Matrix2::new(2.0, 3.0, 4.0, 5.0).determinant();
        let res_det = &ExTensor::new(&[det], &[&[1, 2]]);
        assert_eq!(prod, res_det, "Wedge Product exhibits determinant on F^2x2");
    }

}
