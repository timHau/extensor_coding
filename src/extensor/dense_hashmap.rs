use array_tool::vec::{Intersect, Union};
use num_traits::{One, Zero};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExTensor {
    data: HashMap<Vec<u8>, f64>,
}

impl ExTensor {
    pub(crate) fn new(coeffs: &[f64], basis: &[Vec<u8>]) -> Self {
        assert_eq!(
            basis.len(),
            coeffs.len(),
            "Number of coefficients and basis blades must match"
        );

        let num_elems = basis.len();
        let mut data = HashMap::with_capacity(num_elems);
        data.reserve(num_elems);

        for i in 0..basis.len() {
            data.insert(basis[i].clone(), coeffs[i]);
        }

        ExTensor { data }
    }

    /// this is O(n^2), probably could be better. Where n = number of elements in basis
    pub(crate) fn get_sign_and_ord_basis(a: &Vec<u8>, b: &Vec<u8>) -> (f64, Vec<u8>) {
        let mut ord_basis = a.union(b.clone());

        let mut num_swaps = 0;
        let mut i = 0;
        while i < ord_basis.len() {
            // count number of elements that are smaller than ord_basis[i]
            let mut smaller_count = 0;
            for j in 0..ord_basis.len() {
                if ord_basis[j] < ord_basis[i] {
                    smaller_count += 1;
                }
            }

            // swap ord_basis[i] with ord_basis[smaller_count]
            ord_basis.swap(i, smaller_count);
            num_swaps += 1;

            if i == smaller_count {
                i += 1;
            }
        }

        let sign = if num_swaps % 2 == 0 { 1.0 } else { -1.0 };

        (sign, ord_basis)
    }

    pub(crate) fn lift(&self, k: usize) -> Self {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|(base, coeff)| {
                let shifted: Vec<u8> = base.iter().map(|b| b + k as u8).collect();
                (shifted, coeff)
            })
            .collect();

        self * &ExTensor { data }
    }

    pub(crate) fn coeffs(&self) -> Vec<f64> {
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
            _ => self.data.iter().all(|(_, &coeff)| coeff == 0.0),
        }
    }
}

impl One for ExTensor {
    fn one() -> Self {
        ExTensor::new(&[1.0], &[vec![0]])
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
        let num_elems = self.data.len() * other.data.len();
        let mut data = HashMap::with_capacity(num_elems);
        data.reserve(num_elems);

        for (base_a, coeff_a) in self.data.iter() {
            for (base_b, coeff_b) in other.data.iter() {
                let intersections = base_a.intersect(base_b.clone());
                if intersections.is_empty() {
                    let (sign, next_base) = ExTensor::get_sign_and_ord_basis(base_a, base_b);
                    let next_coeff = sign * coeff_a * coeff_b;

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

impl std::ops::Mul<f64> for &ExTensor {
    type Output = ExTensor;

    fn mul(self, c: f64) -> ExTensor {
        let data = self
            .data
            .iter()
            .map(|(base, coeff)| (base.clone(), coeff.clone() * c))
            .collect();
        ExTensor { data }
    }
}

impl std::ops::Mul<&ExTensor> for f64 {
    type Output = ExTensor;

    fn mul(self, t: &ExTensor) -> ExTensor {
        t * self
    }
}

impl std::ops::Sub for &ExTensor {
    type Output = ExTensor;

    fn sub(self, other: &ExTensor) -> ExTensor {
        self + &(-1.0 * other)
    }
}

impl std::ops::Sub for ExTensor {
    type Output = ExTensor;

    fn sub(self, other: ExTensor) -> ExTensor {
        &self - &other
    }
}

#[cfg(test)]
mod tests {
    use crate::extensor::dense_hashmap::ExTensor;
    use num_traits::Zero;

    #[test]
    fn extensor_add() {
        let x_1 = &ExTensor::new(&[2.0, 5.0], &[vec![1, 3], vec![3, 9]]);
        let x_2 = &ExTensor::new(&[1.0, 1.0], &[vec![1, 2], vec![3, 9]]);
        let sum = x_1 + x_2;
        let res = ExTensor::new(&[2.0, 1.0, 6.0], &[vec![1, 3], vec![1, 2], vec![3, 9]]);
        assert_eq!(&sum, &res, "exterior sum is definined component wise");
        let sum_2 = x_2 + x_1;
        assert_eq!(&sum, &sum_2, "exterior sum is commutative");
    }

    #[test]
    fn get_sign_ord() {
        let x_1 = vec![1, 2, 3];
        let x_2 = vec![4, 5, 6];
        let (sign, ord_basis) = ExTensor::get_sign_and_ord_basis(&x_1, &x_2);
        assert_eq!(sign, 1.0, "sign of simple ordered basis should be 1");
        assert_eq!(
            ord_basis,
            vec![1, 2, 3, 4, 5, 6],
            "ordered basis should match"
        );
    }

    #[test]
    fn get_sign_unord() {
        let x_1 = vec![1, 2, 3];
        let x_2 = vec![4, 6, 5];
        let (sign, ord_basis) = ExTensor::get_sign_and_ord_basis(&x_1, &x_2);
        assert_eq!(sign, -1.0, "sign of simple permutation should be -1");
        assert_eq!(
            ord_basis,
            vec![1, 2, 3, 4, 5, 6],
            "ordered basis should match"
        );
    }

    #[test]
    fn get_sign_unord_2() {
        let x_1 = vec![1, 4, 3];
        let x_2 = vec![2, 6, 5];
        let (sign, ord_basis) = ExTensor::get_sign_and_ord_basis(&x_1, &x_2);
        assert_eq!(sign, 1.0, "sign should be 1");
        assert_eq!(
            ord_basis,
            vec![1, 2, 3, 4, 5, 6],
            "ordered basis should match"
        );
    }

    #[test]
    fn wedge_prod() {
        let x_1 = ExTensor::new(&[2.0, 3.0], &[vec![1, 2], vec![3, 4]]);
        let x_2 = ExTensor::new(&[4.0, 5.0], &[vec![2, 6], vec![4, 7]]);
        let res = ExTensor::new(&[12.0, 10.0], &[vec![2, 3, 4, 6], vec![1, 2, 4, 7]]);
        assert_eq!(&x_1 * &x_2, res, "wedge product should match");
    }

    #[test]
    fn lifted() {
        let x = &ExTensor::new(&[2.0, 3.0], &[vec![1], vec![2]]);
        let l = x.lift(2);
        let a = &ExTensor::new(&[2.0, 3.0], &[vec![3], vec![4]]);
        assert_eq!(l, x * a, "lift is (x, 0)^T wedge (0, x)^T");
    }

    #[test]
    fn extensor_vanish() {
        let x_1 = &ExTensor::new(&[1.0], &[vec![1]]);
        let prod_1 = &(x_1 * x_1);
        assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
    }

    #[test]
    fn extensor_vanish_2() {
        let x_1 = &ExTensor::new(
            &[9.0, 8.0, 7.0, 12.0],
            &[vec![1], vec![1, 2, 3], vec![4], vec![6, 7, 8]],
        );
        let prod_1 = &(x_1 * x_1);
        assert_eq!(prod_1.is_zero(), true, "x wedge x vanishes");
    }

    #[test]
    fn extensor_anti_comm() {
        let x_3 = &ExTensor::new(&[2.0], &[vec![1]]);
        let x_4 = &ExTensor::new(&[4.0], &[vec![3]]);
        let prod_4 = x_3 * x_4;
        let res_1 = ExTensor::new(&[8.0], &[vec![1, 3]]);
        let prod_5 = x_4 * x_3;
        let res_anti = ExTensor::new(&[-8.0], &[vec![1, 3]]);
        assert_eq!(prod_4, res_1, "wedge product on simple extensors");
        assert_eq!(
            prod_5, res_anti,
            "wedge product on simple extensors is anti communative"
        );
    }

    #[test]
    fn det_f2() {
        let x_5 = &ExTensor::new(&[2.0, 3.0], &[vec![1], vec![2]]);
        let x_6 = &ExTensor::new(&[4.0, 5.0], &[vec![1], vec![2]]);
        let prod_6 = x_5 * x_6;
        let det = ExTensor::new(&[-2.0], &[vec![1, 2]]);
        assert_eq!(prod_6, det, "Wedge Product exhibits determinant on F^2x2");
    }

    #[test]
    fn det_f3() {
        let x_7 = &ExTensor::new(&[2.0, 3.0, 4.0], &[vec![1], vec![2], vec![3]]);
        let x_8 = &ExTensor::new(&[5.0, 6.0, 7.0], &[vec![1], vec![2], vec![3]]);
        let x_9 = &ExTensor::new(&[8.0, 9.0, 10.0], &[vec![1], vec![2], vec![3]]);
        let prod_7 = &(&(x_7 * x_8) * x_9);
        let det = &ExTensor::new(&[0.0], &[vec![1, 2, 3]]);
        assert_eq!(prod_7, det, "Wedge Product exhibits determinant on F^3x3");
    }
}
