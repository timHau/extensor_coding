use super::super::utils;
use indexmap::IndexMap;
use std::{
    cmp::PartialEq,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Default)]
pub struct ExTensor {
    data: IndexMap<Vec<i32>, f64>, // basis : coeff
}

/// # ExTensor
///
/// implements an Extensor, which is a mapping from a Vec<i32> (basis) to float (coefficient)
impl ExTensor {
    /// ## new
    ///
    /// create an new Extensor that does not need to be "simple"
    /// meaning sets with cardinality > 1 are supported.
    pub(crate) fn new(coeffs: &[f64], basis: &[&[i32]]) -> Self {
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

    /// ## from
    ///
    /// given an Vec<f64> of coefficients and a Vec of Vec<i32> of basis, create a new extensor.
    pub(crate) fn from(coeffs: Vec<f64>, basis: Vec<Vec<i32>>) -> Self {
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

    /// ## simple
    ///
    /// construct a simple exterior tensor e.g. only using a single basis set
    pub(crate) fn simple(coeff: f64, basis: i32) -> Self {
        let mut data = IndexMap::new();
        data.insert(vec![basis], coeff);
        ExTensor { data }
    }

    /// ## get_sign
    ///
    /// get sign of permutation that brings the basis at 'basis_index' into increasing order
    /// output ∈ {-1, 1}
    fn get_sign(&self, basis_index: usize) -> i32 {
        // from here: https://math.stackexchange.com/questions/65923/how-does-one-compute-the-sign-of-a-permutation
        // get the basis at basis_index
        let v = self.data.get_index(basis_index).unwrap().0;
        // get permutation that would sort that basis
        let perm = utils::get_permutation_to_sort(&v);

        // mark all as not visited
        let mut visited = vec![false; v.len()];
        let mut sign = 1; // initial sign
        for k in 0..v.len() {
            if !visited[k] {
                let mut next = k;
                let mut l = 0;
                while !visited[next] {
                    l += 1;
                    visited[next] = true;
                    next = perm[next];
                }
                if l % 2 == 0 {
                    sign = -sign;
                }
            }
        }

        sign
    }

    /// ## sorted
    ///
    /// sort the basis and apply sign changes if necessary
    pub(crate) fn sorted(&self) -> Self {
        let mut data = IndexMap::new();

        for (i, (basis, coeff)) in self.data.iter().enumerate() {
            let sign = self.get_sign(i) as f64;
            let mut basis_next = basis.to_vec();
            basis_next.sort_unstable();
            if data.contains_key(&basis_next) {
                let coeff_old = data.get(&basis_next).unwrap() + (coeff * sign);
                data.insert(basis_next, coeff_old);
            } else {
                data.insert(basis_next, coeff * sign);
            }
        }

        ExTensor { data }
    }

    /// ## zero
    ///
    /// return the zero / empty ex tensor
    pub(crate) fn zero() -> Self {
        ExTensor::new(&[], &[])
    }

    /// ## is_zero
    ///
    /// test if the extensor is zero, meaning no coeffiecients and no basis or all basis
    /// have zero as a coefficient
    pub(crate) fn is_zero(&self) -> bool {
        match self.data.len() {
            0 => true,
            _ => self.sorted().data.iter().any(|(_, coeff)| *coeff == 0.0),
        }
    }

    /// ## lifted
    ///
    /// calculate the lifted version
    pub(crate) fn lifted(&self) -> Self {
        let mut data = IndexMap::new();
        let n = self.data.len() as i32;
        for (basis, coeff) in self.data.iter() {
            let basis_next: Vec<i32> = basis.iter().map(|v| v + n).collect();
            data.insert(basis_next, coeff.clone());
        }
        self * &ExTensor { data }
    }
}

impl Add<&ExTensor> for &ExTensor {
    type Output = ExTensor;
    fn add(self, rhs: &ExTensor) -> ExTensor {
        let mut data = IndexMap::new();

        let joined_data: Vec<_> = self.data.iter().chain(rhs.data.iter()).collect();
        for val in joined_data {
            let basis = val.0.to_vec();
            let coeff = *val.1;
            if data.contains_key(&basis) {
                let c = data.get(&basis).unwrap() + coeff;
                data.insert(basis, c);
            } else {
                data.insert(basis, coeff);
            }
        }

        ExTensor { data }
    }
}

impl Add<ExTensor> for ExTensor {
    type Output = ExTensor;
    fn add(self, rhs: ExTensor) -> ExTensor {
        &self + &rhs
    }
}

impl Sub<&ExTensor> for &ExTensor {
    type Output = ExTensor;
    fn sub(self, rhs: &ExTensor) -> ExTensor {
        let mut data = IndexMap::new();

        let joined_data: Vec<_> = self.data.iter().chain(rhs.data.iter()).collect();
        for val in joined_data {
            let basis = val.0.to_vec();
            let coeff = *val.1;
            if data.contains_key(&basis) {
                let c = data.get(&basis).unwrap() - coeff;
                data.insert(basis, c);
            } else {
                data.insert(basis, coeff);
            }
        }

        ExTensor { data }
    }
}

impl Sub<ExTensor> for ExTensor {
    type Output = ExTensor;
    fn sub(self, rhs: ExTensor) -> ExTensor {
        &self - &rhs
    }
}

impl Mul<&ExTensor> for &ExTensor {
    type Output = ExTensor;
    fn mul(self, rhs: &ExTensor) -> ExTensor {
        let mut data = IndexMap::new();

        for val_lhs in self.data.iter() {
            for val_rhs in rhs.data.iter() {
                let basis_next: Vec<_> = [&val_lhs.0[..], &val_rhs.0[..]].concat();

                if utils::has_unique_elements(&basis_next) {
                    let coeff_next = val_rhs.1 * val_lhs.1;
                    data.insert(basis_next, coeff_next);
                }
            }
        }

        ExTensor { data }
    }
}

impl Mul<ExTensor> for ExTensor {
    type Output = ExTensor;
    fn mul(self, rhs: ExTensor) -> ExTensor {
        &self * &rhs
    }
}

impl Mul<f64> for ExTensor {
    type Output = ExTensor;
    fn mul(self, c: f64) -> ExTensor {
        let mut data = IndexMap::new();
        for val in self.data {
            data.insert(val.0, val.1 * c);
        }
        ExTensor { data }
    }
}

impl Mul<ExTensor> for f64 {
    type Output = ExTensor;
    fn mul(self, rhs: ExTensor) -> ExTensor {
        let mut data = IndexMap::new();
        for val in rhs.data {
            data.insert(val.0, self * val.1);
        }
        ExTensor { data }
    }
}

impl PartialEq<ExTensor> for ExTensor {
    fn eq(&self, other: &ExTensor) -> bool {
        self.sorted().data == other.sorted().data
    }
}

impl Display for ExTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.data.is_empty() {
            return write!(f, "0");
        }

        for (count_term, d) in self.data.iter().enumerate() {
            if *d.1 < 0.0 {
                s += format!("({}) ", d.1).as_str();
            } else {
                s += format!("{} ", d.1).as_str();
            }

            for (count_base, b) in d.0.iter().enumerate() {
                if count_base == d.0.len() - 1 {
                    if count_term == self.data.len() - 1 {
                        s += format!("e_{}", b).as_str();
                    } else {
                        s += format!("e_{}  +  ", b).as_str();
                    }
                } else {
                    s += format!("e_{} ∧ ", b).as_str();
                }
            }
        }

        write!(f, "{}", s)
    }
}

#[macro_export]
macro_rules! extensor {
    ( $coeff: expr, [$($b: expr),+] ) => {{
        let mut basis = Vec::new();
        $(
            basis.push($b.as_ref());
        )+
        ExTensor::new(&$coeff.as_ref(), &basis)
    }};
}

#[cfg(test)]
mod tests {
    use crate::structure::extensor::ExTensor;

    #[test]
    fn extensor_add() {
        let x_1 = &extensor!([3.0, -7.0], [[1, 3], [3]]);
        let x_2 = &extensor!([1.0, 2.0], [[1], [3]]);
        let sum = x_1 + x_2;
        let res = extensor!([3.0, -5.0, 1.0], [[1, 3], [3], [1]]);
        assert_eq!(sum, res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        assert_eq!(sum, sum_2, "exterior sum is commutative");
    }

    #[test]
    fn extensor_mul_add() {
        let x_1 = &ExTensor::simple(1.0, 1);
        let x_2 = &ExTensor::simple(2.0, 1);
        let x_3 = &ExTensor::simple(1.0, 2);
        let x_4 = &ExTensor::simple(2.0, 2);
        let a = x_1 * x_4 + x_2 * x_1;
        let expect_a = extensor!([2.0], [[1, 2]]);
        let b = x_1 * x_3 + x_2 * x_4;
        let expect_b = extensor!([5.0], [[1, 2]]);
        let c = x_3 * x_4 + x_4 * x_1;
        let expect_c = extensor!([-2.0], [[1, 2]]);
        let d = x_3 * x_3 + x_4 * x_4;
        let expect_d = ExTensor::zero();

        assert_eq!(a, expect_a, "multiplying and then adding (inner product)");
        assert_eq!(b, expect_b, "multiplying and then adding (inner product)");
        assert_eq!(c, expect_c, "multiplying and then adding (inner product)");
        assert_eq!(d, expect_d, "multiplying and then adding (inner product)");
    }

    #[test]
    fn extensor_scalar_mul() {
        let x_1 = extensor!([3.0, 2.0], [[1, 2], [3, 4]]) * 2.0;
        let x_2 = 2.0 * extensor!([3.0, 2.0], [[1, 2], [3, 4]]);
        let res = extensor!([6.0, 4.0], [[1, 2], [3, 4]]);
        assert_eq!(x_1, res, "scalar multiplication is right commutative");
        assert_eq!(x_2, res, "scalar multiplication is left commutative");
        assert_eq!(x_1, x_2, "scalar multiplication is commutative");
    }

    #[test]
    fn extensor_sign() {
        let x_1 = extensor!([-3.0], [[2, 1]]);
        let x_2 = extensor!([3.0], [[1, 2]]);
        assert_eq!(x_1, x_2, "exterior tensors are anti commutative");
    }

    #[test]
    fn extensor_vanish() {
        let x_1 = &ExTensor::simple(1.0, 1);
        let prod_1 = &(x_1 * x_1);
        let zero_tensor = &ExTensor::zero();
        assert_eq!(prod_1, zero_tensor, "x wedge x vanishes");
    }

    #[test]
    fn extensor_anti_comm() {
        // test anti-commutativity
        let x_3 = &ExTensor::simple(2.0, 1);
        let x_4 = &ExTensor::simple(4.0, 3);
        let prod_4 = x_3 * x_4;
        let res_1 = extensor!([8.0], [[1, 3]]);
        let prod_5 = x_4 * x_3;
        let res_anti = extensor!([-8.0], [[1, 3]]);
        assert_eq!(prod_4, res_1, "wedge product on simple extensors");
        assert_eq!(
            prod_5, res_anti,
            "wedge product on simple extensors is anti communative"
        );
    }

    #[test]
    fn det_f2() {
        let x_5 = &extensor!([2.0, 3.0], [[1], [2]]);
        let x_6 = &extensor!([4.0, 5.0], [[1], [2]]);
        let prod_6 = &(x_5 * x_6);
        let det = &extensor!([-2.0], [[1, 2]]);
        assert_eq!(prod_6, det, "Wedge Product exhibits determinant on F^2x2");
    }

    #[test]
    fn det_f3() {
        let x_7 = &extensor!([2.0, 3.0, 4.0], [[1], [2], [3]]);
        let x_8 = &extensor!([5.0, 6.0, 7.0], [[1], [2], [3]]);
        let x_9 = &extensor!([8.0, 9.0, 10.0], [[1], [2], [3]]);
        let prod_7 = &(&(x_7 * x_8) * x_9);
        let det = &extensor!([0.0], [[1, 2, 3]]);
        assert_eq!(prod_7, det, "Wedge Product exhibits determinant on F^3x3");
    }

    #[test]
    fn lifted() {
        let x = extensor!([2.0, 3.0], [[1], [2]]);
        let l = x.lifted();
        let a = extensor!([2.0, 3.0], [[3], [4]]);
        assert_eq!(l, x * a, "lift is (x, 0)^T wedge (0, x)^T");
    }

    #[test]
    fn is_zero() {
        let x = extensor!([0.0, 0.0], [[1, 2, 3], [4, 5, 6]]);
        let y = ExTensor::zero();
        assert_eq!(x.is_zero(), true, "extensor with zero coefficients is zero");
        assert_eq!(y.is_zero(), true, "extensor with empty basis is zero");
    }
}
