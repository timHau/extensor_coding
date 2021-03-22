use super::super::utils;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::{Add, Mul, Sub},
    thread,
};

#[derive(Debug, Clone)]
struct ExTensorComponent {
    basis: Vec<i32>,
    coeff: f64,
}

impl ExTensorComponent {
    fn new(coeff: f64, basis: &[i32]) -> Self {
        // sort the basis and apply a sign if necessary
        let (sign, basis) = Self::get_sign_and_sort(basis.to_vec());
        let coeff = coeff * sign;

        ExTensorComponent { basis, coeff }
    }

    /// ## get_sign_and_sort
    ///
    /// get sign of permutation that brings the basis into increasing order
    /// output ∈ {-1, 1}
    fn get_sign_and_sort(v: Vec<i32>) -> (f64, Vec<i32>) {
        let perm = utils::get_permutation_to_sort(&v);
        let basis = perm.iter().map(|&i| v[i].clone()).collect();

        // mark all as not visited
        // from here: https://math.stackexchange.com/questions/65923/how-does-one-compute-the-sign-of-a-permutation
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

        (sign as f64, basis)
    }

    fn is_zero(&self) -> bool {
        self.basis.len() == 0 || self.coeff == 0.0
    }
}

impl Add<f64> for &ExTensorComponent {
    type Output = ExTensorComponent;
    fn add(self, c: f64) -> ExTensorComponent {
        ExTensorComponent {
            basis: self.basis.to_vec(),
            coeff: self.coeff + c,
        }
    }
}

impl Mul<&ExTensorComponent> for &ExTensorComponent {
    type Output = ExTensorComponent;
    fn mul(self, other: &ExTensorComponent) -> ExTensorComponent {
        let mut basis: Vec<i32> = [&self.basis[..], &other.basis[..]].concat();
        if !utils::has_unique_elements(&basis) {
            // if one basis appears more than once -> zero
            return ExTensorComponent {
                basis: Vec::new(),
                coeff: 0.0,
            };
        }

        let (sign, basis) = ExTensorComponent::get_sign_and_sort(basis);
        let coeff = self.coeff * other.coeff * sign;
        ExTensorComponent { basis, coeff }
    }
}

impl Mul<f64> for &ExTensorComponent {
    type Output = ExTensorComponent;
    fn mul(mut self, c: f64) -> ExTensorComponent {
        ExTensorComponent {
            basis: self.basis.to_vec(),
            coeff: self.coeff * c,
        }
    }
}

impl Hash for ExTensorComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.basis.hash(state);
    }
}

impl PartialEq for ExTensorComponent {
    fn eq(&self, other: &Self) -> bool {
        self.basis == other.basis
    }
}

impl Eq for ExTensorComponent {}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExTensor {
    data: HashSet<ExTensorComponent>,
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
        let mut data = HashSet::new();
        for i in 0..basis.len() {
            let component = ExTensorComponent::new(coeffs[i], basis[i]);
            data.insert(component);
        }
        ExTensor { data }
    }

    /// ## simple
    ///
    /// construct a simple exterior tensor e.g. only using a single basis set
    pub(crate) fn simple(coeff: f64, basis: i32) -> Self {
        let mut data = HashSet::new();
        let component = ExTensorComponent::new(coeff, &[basis]);
        data.insert(component);
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
            _ => self.data.iter().any(|comp| comp.is_zero()),
        }
    }

    /// ## coeffs
    ///
    /// returns only the coefficients of the extensor
    pub(crate) fn coeffs(&self) -> Vec<f64> {
        self.data
            .iter()
            .map(|comp| comp.coeff)
            .collect::<Vec<f64>>()
    }

    /// ## lifted
    ///
    /// calculate the lifted version
    pub(crate) fn lifted(&self) -> Self {
        let mut data = HashSet::new();
        let n = self.data.len() as i32;
        for comp in self.data.iter() {
            let basis_next: Vec<_> = comp.basis.iter().map(|v| v + n).collect();
            let component = ExTensorComponent::new(comp.coeff, &basis_next);
            data.insert(component);
        }
        self * &ExTensor { data }
    }
}

impl Add<&ExTensor> for &ExTensor {
    type Output = ExTensor;
    fn add(self, other: &ExTensor) -> ExTensor {
        let mut data = HashSet::new();

        let joined = self.data.iter().chain(other.data.iter());
        for comp in joined {
            if data.contains(comp) {
                let val: ExTensorComponent = data.get(comp).unwrap() + comp.coeff;
                data.replace(val.clone());
            } else {
                data.insert(comp.clone());
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
        self + &((-1 as f64) * rhs)
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
        let mut data = HashSet::new();
        let mut handles = Vec::with_capacity(self.data.len() * rhs.data.len());

        for comp_lhs in self.data.iter() {
            for comp_rhs in rhs.data.iter() {
                let comp_lhs = comp_lhs.clone();
                let comp_rhs = comp_rhs.clone();
                let handle = thread::spawn(move || &comp_lhs * &comp_rhs);
                handles.push(handle);
            }
        }

        for h in handles {
            let res = h.join().unwrap();
            if !res.is_zero() {
                data.insert(res);
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

impl Mul<f64> for &ExTensor {
    type Output = ExTensor;
    fn mul(self, c: f64) -> ExTensor {
        let mut data = HashSet::new();
        for comp in self.data.iter() {
            data.insert(comp * c);
        }
        ExTensor { data }
    }
}

impl Mul<&ExTensor> for f64 {
    type Output = ExTensor;
    fn mul(self, rhs: &ExTensor) -> ExTensor {
        rhs * self
    }
}

impl Mul<f64> for ExTensor {
    type Output = ExTensor;
    fn mul(self, c: f64) -> ExTensor {
        &self * c
    }
}

impl Mul<ExTensor> for f64 {
    type Output = ExTensor;
    fn mul(self, rhs: ExTensor) -> ExTensor {
        rhs * self
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
    use crate::structure::extensor::{ExTensor, ExTensorComponent};
    use std::collections::HashSet;

    #[test]
    fn hash() {
        let mut h = HashSet::new();
        let x1 = ExTensorComponent::new(1.0, &[1]);
        h.insert(x1);
        let x3 = ExTensorComponent::new(4.0, &[1]);
        assert_eq!(h.contains(&x3), true, "hashing works for ExTensorComponent");
    }

    #[test]
    fn extensor_add() {
        let x_1 = &extensor!([3.0, 7.0], [[1, 3], [3]]);
        let x_2 = &extensor!([1.0, -2.0], [[1], [3]]);
        let sum = x_1 + x_2;
        let res = extensor!([3.0, 5.0, 1.0], [[1, 3], [3], [1]]);
        assert_eq!(sum, res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        println!("{:?}", sum_2);
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
        assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
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
