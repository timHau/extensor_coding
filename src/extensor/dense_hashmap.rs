use array_tool::vec::{Intersect, Union};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ExTensor {
    data: HashMap<Vec<u32>, f64>,
}

impl ExTensor {
    pub(crate) fn new(coeffs: &[f64], basis: &[Vec<u32>]) -> Self {
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
    pub(crate) fn get_sign_and_ord_basis(a: &Vec<u32>, b: &Vec<u32>) -> (f64, Vec<u32>) {
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
                let shifted: Vec<u32> = base.iter().map(|b| b + k as u32).collect();
                (shifted, coeff)
            })
            .collect();

        self * &ExTensor { data }
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

#[cfg(test)]
mod tests {
    use crate::extensor::dense_hashmap::ExTensor;

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
}
