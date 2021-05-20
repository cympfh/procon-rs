#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BitSet(pub u128);

impl BitSet {
    pub fn new() -> Self {
        BitSet(0)
    }
    pub fn from(data: u128) -> Self {
        BitSet(data)
    }
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn contains(&self, i: usize) -> bool {
        (self.0 & (1 << i)) != 0
    }
    pub fn insert(&mut self, i: usize) {
        self.0 |= 1 << i;
    }
    /// subset <: self
    pub fn is_supset_of(&self, subset: &BitSet) -> bool {
        subset.0 & !self.0 == 0
    }
    /// self <: subset
    pub fn is_subset_of(&self, supset: &BitSet) -> bool {
        self.0 & !supset.0 == 0
    }
    pub fn iter(&self) -> BitSetIter {
        BitSetIter(*self, 0)
    }
    pub fn to_vec(&self) -> Vec<usize> {
        self.iter().collect()
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

pub struct BitSetIter(BitSet, usize);
impl std::iter::Iterator for BitSetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let data = (self.0).0;
        let mut cur = self.1;
        while cur < 128 {
            if (data >> cur) & 1 == 1 {
                self.1 = cur + 1;
                return Some(cur);
            }
            cur += 1;
        }
        self.1 = 129;
        None
    }
}

/// Iter of all subsets for {0, 1, .., (n-1)}
pub struct Subsets(u128, u128);
impl Subsets {
    pub fn new(n: usize) -> Self {
        Self(1 << n, 0)
    }
}
impl std::iter::Iterator for Subsets {
    type Item = BitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0 {
            self.1 += 1;
            Some(BitSet::from(self.1 - 1))
        } else {
            None
        }
    }
}

/// Iter of all subsets for all subsets for {0, 1, .., (n-1)}
pub struct SubSubsets(u128, u128, u128);
impl SubSubsets {
    pub fn new(n: usize) -> Self {
        Self(1 << n, 0, 0)
    }
}
impl std::iter::Iterator for SubSubsets {
    type Item = (BitSet, BitSet);
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0 {
            None
        } else if self.2 == 0 {
            self.1 += 1;
            self.2 = self.1;
            Some((BitSet::from(self.1 - 1), BitSet::from(0)))
        } else {
            let set = BitSet::from(self.2);
            self.2 = (self.2 - 1) & self.1;
            Some((BitSet::from(self.1), set))
        }
    }
}

#[macro_export]
macro_rules! bitset {
    ($($elems:expr),* $(,)*) => {{
        #[allow(unused_mut)]
        let mut bs = BitSet::new();
        $(
            bs |= $elems;
        )*
        bs
    }}
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

    #[test]
    fn test_subset() {
        let a = bitset!();
        let b = bitset!(1);
        let c = bitset!(1, 2);
        let d = bitset!(1, 3);
        let e = bitset!(1, 2, 3,);

        assert!(a.is_subset_of(&a));
        assert!(a.is_subset_of(&b));
        assert!(a.is_subset_of(&c));
        assert!(a.is_subset_of(&d));
        assert!(a.is_subset_of(&e));

        assert!(a.is_supset_of(&a));
        assert!(!a.is_supset_of(&b));

        assert!(b.is_subset_of(&c));
        assert!(b.is_subset_of(&d));
        assert!(b.is_subset_of(&e));

        assert!(b.is_supset_of(&b));
        assert!(!b.is_supset_of(&c));

        assert!(!c.is_supset_of(&d));
        assert!(!c.is_subset_of(&d));

        assert!(e.is_supset_of(&a));
        assert!(e.is_supset_of(&b));
        assert!(e.is_supset_of(&c));
        assert!(e.is_supset_of(&d));
        assert!(e.is_supset_of(&e));
    }

    #[test]
    fn test_iter_vec() {
        let a = bitset!();
        let b = bitset!(1);
        let c = bitset!(0, 2);
        let d = bitset!(0, 1, 2);

        assert_eq!(a.to_vec(), vec![]);
        assert_eq!(b.to_vec(), vec![1]);
        assert_eq!(c.to_vec(), vec![0, 2]);
        assert_eq!(d.to_vec(), vec![0, 1, 2]);
    }

    #[test]
    fn test_subsets() {
        assert_eq!(Subsets::new(0).collect::<Vec<_>>(), vec![bitset!()]);
        assert_eq!(
            Subsets::new(1).collect::<Vec<_>>(),
            vec![bitset!(), bitset!(0)]
        );
        assert_eq!(
            Subsets::new(2).collect::<Vec<_>>(),
            vec![bitset!(), bitset!(0), bitset!(1), bitset!(0, 1)]
        );
    }

    #[test]
    fn test_subsubsets() {
        assert_eq!(
            SubSubsets::new(0).collect::<Vec<_>>(),
            vec![(bitset!(), bitset!())]
        );
        assert_eq!(
            SubSubsets::new(1).collect::<Vec<_>>(),
            vec![
                (bitset!(), bitset!()),
                (bitset!(0), bitset!(0)),
                (bitset!(0), bitset!()),
            ]
        );
        assert_eq!(
            SubSubsets::new(2).collect::<Vec<_>>(),
            vec![
                (bitset!(), bitset!()),
                (bitset!(0), bitset!(0)),
                (bitset!(0), bitset!()),
                (bitset!(1), bitset!(1)),
                (bitset!(1), bitset!()),
                (bitset!(0, 1), bitset!(0, 1)),
                (bitset!(0, 1), bitset!(1)),
                (bitset!(0, 1), bitset!(0)),
                (bitset!(0, 1), bitset!()),
            ]
        );
    }

    #[test]
    fn test_is_empty() {
        assert!(BitSet::from(0).is_empty());
        assert!(!BitSet::from(1).is_empty());
    }

    #[test]
    fn test_id() {
        assert_eq!(BitSet::from(0_u128).id(), 0_usize);
        assert_eq!(BitSet::from(31_u128).id(), 31_usize);
    }
}
