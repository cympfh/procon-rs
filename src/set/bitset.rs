#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSet(pub u128);

impl BitSet {
    pub fn new() -> Self {
        BitSet(0)
    }
    pub fn from(data: u128) -> Self {
        BitSet(data)
    }
    pub fn contains(&self, i: usize) -> bool {
        (self.0 & (1 << i)) != 0
    }
    pub fn insert(&mut self, i: usize) {
        self.0 |= 1 << i;
    }
}

impl std::ops::BitAnd<usize> for BitSet {
    type Output = bool;
    fn bitand(self, i: usize) -> bool {
        self.contains(i)
    }
}

impl std::ops::BitOr<usize> for BitSet {
    type Output = Self;
    fn bitor(self, i: usize) -> Self {
        BitSet(self.0 | (1 << i))
    }
}

impl std::ops::BitOrAssign<usize> for BitSet {
    fn bitor_assign(&mut self, i: usize) {
        self.insert(i);
    }
}

#[cfg(test)]
mod test_bitset {
    use crate::set::bitset::*;

    #[test]
    fn test() {
        let mut bs = BitSet::new();
        for i in 0..128 {
            assert!(!bs.contains(i));
        }
        bs.insert(5);
        assert!(bs.contains(5));
        bs.insert(7);
        assert!(bs.contains(5));
        assert!(bs.contains(7));
    }

    #[test]
    fn test_ops() {
        let mut bs = BitSet::from(3);
        assert!(bs & 0);
        assert!(bs & 1);
        assert!(!(bs & 2));
        bs |= 2;
        assert!(bs & 2);
    }

    #[test]
    fn test_immutable() {
        let bs = BitSet::new();
        let cs = bs | 0 | 2 | 4 | 6;
        for i in 0..6 {
            assert!(!bs.contains(i));
            if i % 2 == 0 {
                assert!(cs.contains(i));
            } else {
                assert!(!cs.contains(i));
            }
        }
    }
}
