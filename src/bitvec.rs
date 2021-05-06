use std::cmp::max;
use std::hash::Hasher;

#[derive(Debug, Eq, Clone)]
pub(crate) struct BitVec {
    data: Vec<bool>,
}

impl BitVec {
    pub(crate) fn from(v: &Vec<u8>) -> Self {
        let max_elem = v.iter().max().unwrap_or(&0);
        let mut data = vec![false; 2 * (*max_elem as usize)];

        for i in v.iter() {
            if *i > 0 {
                data[(*i - 1) as usize] = true;
            }
        }

        BitVec { data }
    }

    pub(crate) fn shift_right(&mut self, by: usize) {
        let mut pre = vec![false; by];

        for b in self.data.iter() {
            pre.push(*b);
        }

        self.data = pre;
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn count_ones(&self) -> u32 {
        let mut sum = 0;

        for b in self.data.iter() {
            if *b {
                sum += 1;
            }
        }

        sum
    }

    pub(crate) fn any(&self) -> bool {
        for b in self.data.iter() {
            if *b {
                return true;
            }
        }

        false
    }
}

impl std::ops::BitXor for &BitVec {
    type Output = BitVec;

    fn bitxor(self, rhs: &BitVec) -> BitVec {
        let mut data = vec![];

        let max_len = max(self.data.len(), rhs.data.len());
        for i in 0..max_len {
            let b_1 = self.data.get(i).unwrap_or(&false);
            let b_2 = rhs.data.get(i).unwrap_or(&false);

            if (!b_1 && *b_2) || (*b_1 && !b_2) {
                data.push(true)
            } else {
                data.push(false);
            }
        }

        BitVec { data }
    }
}

impl std::ops::BitAnd for &BitVec {
    type Output = BitVec;

    fn bitand(self, rhs: &BitVec) -> BitVec {
        let mut data = vec![];

        let max_len = max(self.data.len(), rhs.data.len());
        for i in 0..max_len {
            let b_1 = self.data.get(i).unwrap_or(&false);
            let b_2 = rhs.data.get(i).unwrap_or(&false);

            if *b_1 && *b_2 {
                data.push(true)
            } else {
                data.push(false);
            }
        }

        BitVec { data }
    }
}

impl std::ops::BitAnd for BitVec {
    type Output = BitVec;

    fn bitand(self, rhs: BitVec) -> BitVec {
        &self & &rhs
    }
}

impl std::cmp::PartialEq for BitVec {
    fn eq(&self, rhs: &BitVec) -> bool {
        for (b1, b2) in self.data.iter().zip(rhs.data.iter()) {
            if *b1 != *b2 {
                return false;
            }
        }

        true
    }
}

impl std::hash::Hash for BitVec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl std::fmt::Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from("[");

        for b in self.data.iter() {
            if *b {
                res += "1";
            } else {
                res += "0";
            }
        }

        res += "]";

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitvec::BitVec;

    #[test]
    fn create() {
        let v = vec![1, 3, 9, 12];
        let b = BitVec::from(&v);
        let res = vec![
            true, false, true, false, false, false, false, false, true, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false,
        ];
        println!("b: {:?}", b);
        assert_eq!(b.data, res, "bitvec init should work");
    }

    #[test]
    fn bitxor() {
        let b_1 = &BitVec::from(&vec![1, 3, 6]);
        let b_2 = &BitVec::from(&vec![1, 2, 3, 4, 6]);
        let res = b_1 ^ b_2;
        let expect = BitVec::from(&vec![2, 4]);
        assert_eq!(res, expect, "bitwise xor should work");
    }

    #[test]
    fn bitxor_2() {
        let b_1 = &BitVec::from(&vec![1]);
        let b_2 = &BitVec::from(&vec![10]);
        let res = b_1 ^ b_2;
        let expect = BitVec::from(&vec![1, 10]);
        assert_eq!(res, expect, "bitwise xor should work");
    }

    #[test]
    fn bitand() {
        let b_1 = &BitVec::from(&vec![1, 3, 6]);
        let b_2 = &BitVec::from(&vec![1, 2, 3, 4, 6]);
        let res = b_1 & b_2;
        let expect = BitVec::from(&vec![1, 3, 6]);
        assert_eq!(res, expect, "bitwise and should work");
    }
    #[test]
    fn shift() {
        let mut b_1 = BitVec::from(&vec![1, 3, 6]);
        b_1.shift_right(3);
        let res = BitVec::from(&vec![4, 6, 9]);
        assert_eq!(b_1, res, "shift_right should work");
    }

    #[test]
    fn any() {
        let b_1 = BitVec::from(&vec![10]);
        assert_eq!(b_1.any(), true, "should be true if at least one bit is set");
        let b_2 = BitVec::from(&vec![1, 10, 100]);
        assert_eq!(b_2.any(), true, "should be true if at least one bit is set");
        let b_3 = BitVec::from(&vec![]);
        assert_eq!(b_3.any(), false, "should be false if no bit is set");
    }

    #[test]
    fn count() {
        let b_1 = BitVec::from(&vec![10]);
        assert_eq!(b_1.count_ones(), 1, "should count ones");
        let b_2 = BitVec::from(&vec![1, 3, 4, 5, 7, 10]);
        assert_eq!(b_2.count_ones(), 6, "should count ones");
    }
}
