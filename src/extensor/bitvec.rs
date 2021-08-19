use crate::bitvec::BitVec;
use num_traits::{One, Zero};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ExTensor {
    data: HashMap<BitVec, i64>,
}

/// # ExTensor
///
/// Given an array of i64 coefficients `coeffs` and a slice of vectors `basis` construct an extensor.
/// An ExTensor is represented as an hash map from bitvec (basis) to i64 (coefficient).
/// The bit coding in the basis is as follows
///
/// |  binary  |       basis     |
/// |----------|-----------------|
/// | 00000000 |               1 |
/// | 10000000 |             e_1 |
/// | 11000000 |       e_1 ∧ e_2 |
/// | 00100000 |             e_3 |
/// | 10100000 |       e_1 ∧ e_3 |
/// | 11100000 | e_1 ∧ e_2 ∧ e_3 |
impl ExTensor {
    /// ## new
    ///
    /// Given an Slice of i64 and a Slice of Basis Vecs (u8) create a new ExTensor
    pub fn new(coeffs: &[i64], basis: &[Vec<u8>]) -> Self {
        assert_eq!(
            basis.len(),
            coeffs.len(),
            "Number of coefficients and basis blades must match"
        );

        let mut data = HashMap::with_capacity(basis.len());
        for (i, b) in basis.iter().enumerate() {
            let base = BitVec::from(b);
            data.insert(base, coeffs[i]);
        }

        ExTensor { data }
    }

    /// ## get_sign
    ///
    /// Given the basis representation `a` and `b` of two ExTensors determine
    /// the sign of the permutation that will sort the union of `a` and `b`
    pub(crate) fn get_sign(a: &BitVec, b: &BitVec) -> i64 {
        let mut num_perm = 0;

        let indices_a = a.indices();
        let indices_b = b.indices();

        let mut i = 0;
        let mut j = 0;
        while i < indices_a.len() && j < indices_b.len() {
            if indices_a[i] <= indices_b[j] {
                i += 1;
            } else {
                j += 1;
                num_perm += indices_a.len() - i;
            }
        }

        if num_perm % 2 == 0 {
            1
        } else {
            -1
        }
    }

    /// ## lift
    ///
    /// Lift an ExTensor `self`, wich means to "shift" the basis by `k` to obtain a new
    /// extensor `self'` and calculate `self wedge self'` more formal
    /// ```not-a-test
    /// ∑_(i in {1..k}) a_i e_i   ∧   ∑_(j in {1..k}) a_j e_(j + k)
    /// ```
    pub(crate) fn lift(&self, k: usize) -> Self {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|(mut base, coeff)| {
                base.shift_right(k);
                (base, coeff)
            })
            .collect();
        self * &ExTensor { data }
    }

    /// ## coeffs
    ///
    /// Return the coefficients of the ExTensor
    pub fn coeffs(&self) -> Vec<i64> {
        self.data.iter().map(|(_, coeff)| coeff.clone()).collect()
    }
}

impl Zero for ExTensor {
    fn zero() -> Self {
        ExTensor {
            data: HashMap::new(),
        }
    }

    fn is_zero(&self) -> bool {
        match self.data.len() {
            0 => true,
            _ => self.data.iter().all(|(_, &coeff)| coeff == 0),
        }
    }
}

impl One for ExTensor {
    fn one() -> Self {
        ExTensor::new(&[1], &[vec![0]])
    }
}

impl std::ops::Add for &ExTensor {
    type Output = ExTensor;

    fn add(self, other: &ExTensor) -> ExTensor {
        let joined_data = self.data.iter().chain(other.data.iter());

        let mut data = HashMap::with_capacity(self.data.len() + other.data.len());
        for (base, coeff) in joined_data {
            if data.contains_key(base) {
                let next_coeff: i64 = data.get(base).unwrap() + coeff;
                data.insert(base.clone(), next_coeff);
            } else {
                data.insert(base.clone(), *coeff);
            }
        }

        ExTensor { data }
    }
}

impl std::ops::Add for ExTensor {
    type Output = ExTensor;

    fn add(self, other: ExTensor) -> ExTensor {
        &self + &other
    }
}

impl std::ops::Mul for &ExTensor {
    type Output = ExTensor;

    fn mul(self, other: &ExTensor) -> ExTensor {
        let num_elems = self.data.len() * other.data.len();
        let mut data = HashMap::with_capacity(num_elems);
        data.reserve(num_elems);

        for (base_a, coeff_a) in self.data.iter() {
            for (base_b, coeff_b) in other.data.iter() {
                // check if the base is independent. Intersection test can be done via bitwise and
                // only if they are independent (no common basis element) will we continue.
                let intersections = base_a & base_b;
                if !intersections.any() {
                    // calculate the next basis bit vec, which can be done via bitwise or
                    let next_base = base_a ^ base_b;
                    // compute sign and multiply coefficients
                    let sign = ExTensor::get_sign(base_a, base_b);
                    let next_coeff: i64 = sign * coeff_a * coeff_b;

                    if data.contains_key(&next_base) {
                        let old_coeff = data[&next_base];
                        let next_coeff = old_coeff + next_coeff;
                        data.insert(next_base, next_coeff);
                    } else {
                        data.insert(next_base, next_coeff);
                    }
                }
            }
        }

        ExTensor { data }
    }
}

impl std::ops::Mul for ExTensor {
    type Output = ExTensor;

    fn mul(self, other: ExTensor) -> ExTensor {
        &self * &other
    }
}

impl std::fmt::Display for ExTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from("\n");

        for (i, (base, coeff)) in self.data.iter().enumerate() {
            if coeff != &0 {
                res += &format!("{} {}", coeff, base);
                if i < self.data.len() - 1 {
                    res += "+";
                }
            }
        }

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitvec::BitVec;
    use crate::extensor::bitvec::ExTensor;
    use num_traits::Zero;

    #[test]
    fn extensor_add() {
        let x_1 = &ExTensor::new(&[2, 5], &[vec![1, 3], vec![3, 9]]);
        let x_2 = &ExTensor::new(&[1, 1], &[vec![1, 2], vec![3, 9]]);
        let sum = x_1 + x_2;
        let res = &ExTensor::new(&[2, 1, 6], &[vec![1, 3], vec![1, 2], vec![3, 9]]);
        assert_eq!(&sum, res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        assert_eq!(&sum, &sum_2, "exterior sum is commutative");
    }

    #[test]
    fn extensor_add_2() {
        let x_1 = &ExTensor::new(&[-3, 4], &[vec![1, 3], vec![3, 9]]);
        let x_2 = &ExTensor::new(&[3, -4], &[vec![1, 3], vec![3, 9]]);
        let sum = x_1 + x_2;
        let res = &ExTensor::new(&[0, 0], &[vec![1, 3], vec![3, 9]]);
        assert_eq!(&sum, res, "tensors should cancel each other");
    }

    #[test]
    fn extensor_add_3() {
        let x_1 = &ExTensor::new(&[-3, 4], &[vec![1, 3], vec![3, 4]]);
        let x_2 = &ExTensor::new(&[3, -4], &[vec![1, 3], vec![3, 9]]);
        let sum = x_1 + x_2;
        let res = &ExTensor::new(&[0, 4, -4], &[vec![1, 3], vec![3, 4], vec![3, 9]]);
        assert_eq!(&sum, res, "tensors should add");
    }

    #[test]
    fn get_sign() {
        let x_1 = BitVec::from(&vec![2]);
        let x_2 = BitVec::from(&vec![2]);
        assert_eq!(ExTensor::get_sign(&x_1, &x_2), 1);
        let x_3 = BitVec::from(&vec![3]);
        assert_eq!(ExTensor::get_sign(&x_1, &x_3), 1);
        let x_4 = BitVec::from(&vec![3, 4]);
        assert_eq!(ExTensor::get_sign(&x_1, &x_4), 1);
        let x_5 = BitVec::from(&vec![3, 4, 5]);
        assert_eq!(ExTensor::get_sign(&x_1, &x_5), 1);
    }

    #[test]
    fn get_sign_2() {
        let x_1 = BitVec::from(&vec![1, 2, 4]);
        let x_2 = BitVec::from(&vec![3, 5, 6]);
        let sign = ExTensor::get_sign(&x_1, &x_2);
        assert_eq!(sign, -1, "sign of simple permutation should be -1");
    }

    #[test]
    fn lifted() {
        let x = &ExTensor::new(&[2, 3], &[vec![1], vec![2]]);
        let l = x.lift(2);
        let a = &ExTensor::new(&[2, 3], &[vec![3], vec![4]]);
        // (2 e_1 + 3 e_2) ^ (2 e_3 + 3 e_4) = 4 e_1 ^ e_3 + 6 e_2 ^ e_3  + 6 e_1 ^ e_4 + 9 e_2 ^ e_4
        // (2, 3, 0, 0).T ^ (0, 0, 2, 3).T = (2 e_1 + 3 e_2 + 0 e_3 + 0_e_4) ^ (0 e_1 + 0 e_2 + 2 e_3 + 3 e_4)
        assert_eq!(l, x * a, "lift is (x, 0)^T wedge (0, x)^T");
    }

    #[test]
    fn wedge_prod() {
        let x_1 = ExTensor::new(&[2, 3], &[vec![1, 2], vec![3, 4]]);
        let x_2 = ExTensor::new(&[4, 5], &[vec![2, 6], vec![4, 7]]);
        let res = ExTensor::new(&[12, 10], &[vec![2, 3, 4, 6], vec![1, 2, 4, 7]]);
        assert_eq!(&x_1 * &x_2, res, "wedge product should match");
    }

    #[test]
    fn wedge_prod_2() {
        let x_1 = ExTensor::new(&[3], &[vec![3, 4]]);
        let x_2 = ExTensor::new(&[4], &[vec![2, 6]]);
        let res = ExTensor::new(&[12], &[vec![2, 3, 4, 6]]);
        assert_eq!(&x_1 * &x_2, res, "wedge product should match");
    }

    #[test]
    fn extensor_mul_add() {
        let x_1 = &ExTensor::new(&[1], &[vec![1]]);
        let x_2 = &ExTensor::new(&[2], &[vec![1]]);
        let x_3 = &ExTensor::new(&[1], &[vec![2]]);
        let x_4 = &ExTensor::new(&[2], &[vec![2]]);
        let a = x_1 * x_4 + x_2 * x_1;
        let expect_a = ExTensor::new(&[2], &[vec![1, 2]]);
        let b = x_1 * x_3 + x_2 * x_4;
        let expect_b = ExTensor::new(&[5], &[vec![1, 2]]);
        let c = x_3 * x_4 + x_4 * x_1;
        let expect_c = ExTensor::new(&[-2], &[vec![1, 2]]);
        let d = x_3 * x_3 + x_4 * x_4;
        let expect_d = ExTensor::zero();

        assert_eq!(a, expect_a, "multiplying and then adding (inner product)");
        assert_eq!(b, expect_b, "multiplying and then adding (inner product)");
        assert_eq!(c, expect_c, "multiplying and then adding (inner product)");
        assert_eq!(d, expect_d, "multiplying and then adding (inner product)");
    }

    #[test]
    fn extensor_vanish() {
        let x_1 = &ExTensor::new(&[1], &[vec![1]]);
        let prod_1 = &(x_1 * x_1);
        assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
    }

    #[test]
    fn extensor_vanish_2() {
        let x_1 = &ExTensor::new(
            &[9, 8, 7, 12],
            &[vec![1], vec![1, 2, 3], vec![4], vec![6, 7, 8]],
        );
        let prod_1 = &(x_1 * x_1);
        assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
    }

    #[test]
    fn extensor_anti_comm() {
        // test anti-commutativity
        let x_3 = &ExTensor::new(&[2], &[vec![1]]);
        let x_4 = &ExTensor::new(&[4], &[vec![3]]);
        let prod_4 = x_3 * x_4;
        let res_1 = ExTensor::new(&[8], &[vec![1, 3]]);
        let prod_5 = x_4 * x_3;
        let res_anti = ExTensor::new(&[-8], &[vec![1, 3]]);
        assert_eq!(prod_4, res_1, "wedge product on simple extensors");
        assert_eq!(
            prod_5, res_anti,
            "wedge product on simple extensors is anti communative"
        );
    }

    #[test]
    fn det_f2() {
        let x_5 = &ExTensor::new(&[2, 3], &[vec![1], vec![2]]);
        let x_6 = &ExTensor::new(&[4, 5], &[vec![1], vec![2]]);
        let prod_6 = &(x_5 * x_6);
        let det = &ExTensor::new(&[-2], &[vec![1, 2]]);
        assert_eq!(prod_6, det, "Wedge Product exhibits determinant on F^2x2");
    }

    #[test]
    fn det_f3() {
        let x_7 = &ExTensor::new(&[2, 3, 4], &[vec![1], vec![2], vec![3]]);
        let x_8 = &ExTensor::new(&[5, 6, 7], &[vec![1], vec![2], vec![3]]);
        let x_9 = &ExTensor::new(&[8, 9, 10], &[vec![1], vec![2], vec![3]]);
        let prod_7 = &(&(x_7 * x_8) * x_9);
        let det = &ExTensor::new(&[0], &[vec![1, 2, 3]]);
        assert_eq!(prod_7, det, "Wedge Product exhibits determinant on F^3x3");
    }

    #[test]
    fn is_zero() {
        let x = ExTensor::new(&[0, 0], &[vec![1, 2, 3], vec![4, 5, 6]]);
        let y = ExTensor::zero();
        assert_eq!(x.is_zero(), true, "extensor with zero coefficients is zero");
        assert_eq!(y.is_zero(), true, "extensor with empty basis is zero");
    }
}
