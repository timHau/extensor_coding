use bitvec::prelude::{bitvec, BitVec};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ExTensor {
    data: HashMap<BitVec, f64>,
}

/// # ExTensor
///
/// implements an Extensor
impl ExTensor {
    pub(crate) fn new(coeffs: &[f64], basis: &[Vec<u32>]) -> Self {
        assert_eq!(
            basis.len(),
            coeffs.len(),
            "Number of coefficients and basis blades must match"
        );

        let max_len = 64;
        let mut data = HashMap::with_capacity(basis.len());
        for (i, b) in basis.iter().enumerate() {
            let mut base = bitvec![0; max_len];
            for bv in b {
                if bv <= &(max_len as u32) {
                    base.set((*bv) as usize, true);
                }
            }

            data.insert(base, coeffs[i]);
        }

        ExTensor { data }
    }

    pub(crate) fn get_sign(a: &BitVec, b: &BitVec) -> f64 {
        let mut sum = 0;

        let mut c = a.clone();
        c.shift_right(1);
        while c.any() {
            sum += (c.clone() & b.clone()).count_ones();
            c.shift_right(1);
        }

        if sum % 2 == 0 {
            1.0
        } else {
            -1.0
        }
    }

    pub(crate) fn zero() -> Self {
        ExTensor {
            data: HashMap::new(),
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.data.len() == 0
    }
}

impl std::ops::Add for &ExTensor {
    type Output = ExTensor;
    fn add(self, other: &ExTensor) -> ExTensor {
        let joined_data = self.data.iter().chain(other.data.iter());

        let mut data = HashMap::with_capacity(self.data.len() + other.data.len());
        for (base, coeff) in joined_data {
            if data.contains_key(base) {
                let next_coeff: f64 = data.get(base).unwrap() + coeff;
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
        let mut data = HashMap::with_capacity(self.data.len() * other.data.len());

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
                    let next_coeff: f64 = sign * coeff_a * coeff_b;
                    
                    if data.contains_key(&next_base) {
                        let old_coeff = data.get(&next_base).unwrap();
                        data.insert(next_base, old_coeff + next_coeff);
                    } else {
                        data.insert(next_base, next_coeff);
                    }
                }
            }
        }

        ExTensor { data }
    }
}

impl std::ops::Mul<f64> for ExTensor {
    type Output = ExTensor;
    fn mul(self, c: f64) -> ExTensor {
        let data = self
            .data
            .into_iter()
            .map(|(base, coeff)| (base, coeff * c))
            .collect();
        ExTensor { data }
    }
}

impl std::ops::Mul<ExTensor> for f64 {
    type Output = ExTensor;
    fn mul(self, t: ExTensor) -> ExTensor {
        t * self
    }
}

impl std::fmt::Display for ExTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from("");

        for (i, (base, coeff)) in self.data.iter().enumerate() {
            res += &format!("{}", coeff);
            for (j, b) in base.iter().enumerate() {
                if *b {
                    res += &format!("e_{} ∧ ", j);
                }
            }
            if i < self.data.len() - 1 {
                res += " + ";
            }
        }

        write!(f, "{}", res)
    }
}

#[macro_export]
macro_rules! extensor {

    ($coeffs:expr, [$($b: expr),*] ) => {{
        let mut basis = Vec::new();
        $(
           basis.push($b.to_vec());
        )*
        ExTensor::new($coeffs.as_ref(), &basis)
    }};

}

#[cfg(test)]
mod tests {
    use crate::structure::extensor::ExTensor;

    #[test]
    fn extensor_add() {
        let x_1 = &extensor!([2.0, 5.0], [[1, 3], [3, 9]]);
        let x_2 = &extensor!([1.0, 1.0], [[1, 2], [3, 9]]);
        let sum = x_1 + x_2;
        let res = &extensor!([2.0, 1.0, 6.0], [[1, 3], [1, 2], [3, 9]]);
        assert_eq!(&sum, res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        assert_eq!(&sum, &sum_2, "exterior sum is commutative");
    }

    #[test]
    fn wedge_prod() {
        let x_1 = &extensor!([2.0, 3.0], [[1, 2], [3, 4]]);
        let x_2 = &extensor!([4.0, 5.0], [[6, 2], [7, 4]]);
        let res_1 = &extensor!([12.0, 10.0], [[2, 3, 4, 6], [1, 2, 4, 7]]);
        assert_eq!(&(x_1 * x_2), res_1, "wedge product");
        let x_3 = &extensor!([1.0], [[2]]);
        let x_4 = &extensor!([1.0], [[1]]);
        let res_2 = &extensor!([-1.0], [[1, 2]]);
        assert_eq!(
            &(x_3 * x_4),
            res_2,
            "sign changes when base has to be reorderd"
        );
    }

    #[test]
    fn extensor_mul_add() {
        let x_1 = &extensor!([1.0], [[1]]);
        let x_2 = &extensor!([2.0], [[1]]);
        let x_3 = &extensor!([1.0], [[2]]);
        let x_4 = &extensor!([2.0], [[2]]);
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
        fn extensor_vanish() {
            let x_1 = &extensor!([1.0], [[1]]);
            let prod_1 = &(x_1 * x_1);
            assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
        }

        #[test]
        fn extensor_anti_comm() {
            // test anti-commutativity
            let x_3 = &extensor!([2.0], [[1]]);
            let x_4 = &extensor!([4.0], [[3]]);
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

    /*
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
    */
}
