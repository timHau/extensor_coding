use bitvec::prelude::{bitvec, BitVec};
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
    pub(crate) fn new(coeffs: &[i64], basis: &[Vec<u8>]) -> Self {
        assert_eq!(
            basis.len(),
            coeffs.len(),
            "Number of coefficients and basis blades must match"
        );
        let max_basis_len = 2 * 8;

        let mut data = HashMap::with_capacity(basis.len());
        for (i, b) in basis.iter().enumerate() {
            let mut base = bitvec![0; max_basis_len];
            for bv in b {
                if bv <= &(max_basis_len as u8) {
                    base.set((*bv - 1) as usize, true);
                } else {
                    panic!(
                        "To many basis elements for extensor, max_len is {}",
                        max_basis_len
                    );
                }
            }

            data.insert(base, coeffs[i]);
        }

        ExTensor { data }
    }

    pub(crate) fn get_sign(a: &BitVec, b: &BitVec) -> i64 {
        let mut sum: u32 = 0;

        for i in 1..a.len() - 1 {
            let b = b.clone();
            let mut a = a.clone();
            a.shift_right(i);
            sum += (a & b).count_ones() as u32;
        }

        if sum % 2 == 0 {
            1
        } else {
            -1
        }
    }

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

    #[allow(dead_code)]
    pub(crate) fn coeffs(&self) -> Vec<i64> {
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
                let intersections = base_a.clone() & base_b.clone();
                if !intersections.any() {
                    // calculate the next basis bit vec, which can be done via bitwise or
                    let next_base = base_a.clone() ^ base_b.clone();
                    // compute sign and multiply coefficients
                    let sign = ExTensor::get_sign(base_b, base_a);
                    let next_coeff: i64 = sign * coeff_a * coeff_b;

                    if data.contains_key(&next_base) {
                        let old_coeff = data.get(&next_base).unwrap();
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

impl std::ops::Mul<i64> for &ExTensor {
    type Output = ExTensor;

    fn mul(self, c: i64) -> ExTensor {
        let data = self
            .data
            .iter()
            .map(|(base, coeff)| (base.clone(), coeff.clone() * c))
            .collect();
        ExTensor { data }
    }
}

impl std::ops::Mul<&ExTensor> for i64 {
    type Output = ExTensor;

    fn mul(self, t: &ExTensor) -> ExTensor {
        t * self
    }
}

impl std::ops::Sub for &ExTensor {
    type Output = ExTensor;

    fn sub(self, other: &ExTensor) -> ExTensor {
        self + &(-1 * other)
    }
}

impl std::ops::Sub for ExTensor {
    type Output = ExTensor;

    fn sub(self, other: ExTensor) -> ExTensor {
        &self - &other
    }
}

impl std::fmt::Display for ExTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from("");

        for (i, (base, coeff)) in self.data.iter().enumerate() {
            if coeff != &0 {
                res += &format!("{} ", coeff);
                for (j, b) in base.iter().enumerate() {
                    if *b {
                        if j < base.len() {
                            res += &format!("e_{} ∧ ", j + 1);
                        } else {
                            res += &format!("e_{}", j + 1);
                        }
                    }
                }
                if i < self.data.len() - 1 {
                    res += " + ";
                }
            }
        }

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::extensor::bitvec::ExTensor;
    use bitvec::prelude::{bitvec, Lsb0};
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
    fn wedge_prod() {
        let x_1 = &ExTensor::new(&[2, 3], &[vec![1, 2], vec![3, 4]]);
        let x_2 = &ExTensor::new(&[4, 5], &[vec![6, 2], vec![7, 4]]);
        let res_1 = &ExTensor::new(&[12, 10], &[vec![2, 3, 4, 6], vec![1, 2, 4, 7]]);
        assert_eq!(&(x_1 * x_2), res_1, "wedge product");
        let x_3 = &ExTensor::new(&[1], &[vec![2]]);
        let x_4 = &ExTensor::new(&[1], &[vec![1]]);
        let res_2 = &ExTensor::new(&[-1], &[vec![1, 2]]);
        assert_eq!(
            &(x_3 * x_4),
            res_2,
            "sign changes when base has to be reorderd"
        );
    }

    #[test]
    fn get_sign() {
        let x_1 = bitvec![0, 1, 0, 0, 0];
        let x_2 = bitvec![0, 1, 0, 0, 0];
        assert_eq!(ExTensor::get_sign(&x_1, &x_2), 1);
        let x_3 = bitvec![0, 0, 1, 0, 0];
        assert_eq!(ExTensor::get_sign(&x_1, &x_3), -1);
        let x_4 = bitvec![0, 0, 1, 1, 0];
        assert_eq!(ExTensor::get_sign(&x_1, &x_4), 1);
        let x_5 = bitvec![0, 0, 1, 1, 1];
        assert_eq!(ExTensor::get_sign(&x_1, &x_5), -1);
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
    fn extensor_scalar_mul() {
        let x_1 = &ExTensor::new(&[3, 2], &[vec![1, 2], vec![3, 4]]) * 2;
        let x_2 = 2 * &ExTensor::new(&[3, 2], &[vec![1, 2], vec![3, 4]]);
        let res = ExTensor::new(&[6, 4], &[vec![1, 2], vec![3, 4]]);
        assert_eq!(x_1, res, "scalar multiplication is right commutative");
        assert_eq!(x_2, res, "scalar multiplication is left commutative");
        assert_eq!(x_1, x_2, "scalar multiplication is commutative");
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
    fn lifted() {
        let x = &ExTensor::new(&[2, 3], &[vec![1], vec![2]]);
        let l = x.lift(2);
        let a = &ExTensor::new(&[2, 3], &[vec![3], vec![4]]);
        assert_eq!(l, x * a, "lift is (x, 0)^T wedge (0, x)^T");
    }

    #[test]
    fn is_zero() {
        let x = ExTensor::new(&[0, 0], &[vec![1, 2, 3], vec![4, 5, 6]]);
        let y = ExTensor::zero();
        assert_eq!(x.is_zero(), true, "extensor with zero coefficients is zero");
        assert_eq!(y.is_zero(), true, "extensor with empty basis is zero");
    }
}
