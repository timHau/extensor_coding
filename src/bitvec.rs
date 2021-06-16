use std::hash::Hasher;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct BitVec {
    data: [bool; 32],
}

impl BitVec {
    pub(crate) fn from(v: &Vec<u8>) -> Self {
        let mut data = [false; 32];

        for i in v.iter() {
            data[*i as usize] = true;
        }

        BitVec { data }
    }

    pub(crate) fn shift_right(&mut self, k: usize) {
        self.data.rotate_right(k)
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

    pub(crate) fn indices(&self) -> Vec<usize> {
        let mut res = vec![];

        for (i, b) in self.data.iter().enumerate() {
            if *b {
                res.push(i);
            }
        }

        res
    }
}

impl std::ops::BitXor for &BitVec {
    type Output = BitVec;

    fn bitxor(self, rhs: &BitVec) -> BitVec {
        let mut data = [false; 32];

        for i in 0..32 {
            let b_1 = self.data[i];
            let b_2 = rhs.data[i];
            data[i] = (!b_1 && b_2) || (b_1 && !b_2);
        }

        BitVec { data }
    }
}

impl std::ops::BitAnd for &BitVec {
    type Output = BitVec;

    fn bitand(self, rhs: &BitVec) -> BitVec {
        let mut data = [false; 32];

        for i in 0..32 {
            let b_1 = self.data[i];
            let b_2 = rhs.data[i];

            data[i] = b_1 && b_2;
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

impl std::hash::Hash for BitVec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl std::ops::Index<usize> for BitVec {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        self.data.get(index).unwrap()
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
        let res = [
            false, true, false, true, false, false, false, false, false, true, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false,
        ];
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
        let b_2 = BitVec::from(&vec![1, 10, 20]);
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
