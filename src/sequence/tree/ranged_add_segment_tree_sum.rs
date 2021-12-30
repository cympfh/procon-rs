/// Sequence - Lazy Segment Tree - Ranged Add/Segment Tree of Sum
use crate::algebra::act::*;
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE
use crate::sequence::tree::lazy_segment_tree::*;

pub struct RangedAddSegmentTreeSum {
    pub t: LazySegmentTree<CountedSum, CountedAdd>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountedSum(pub i64, pub u64);
monoid! {
    CountedSum;
    one = CountedSum(0, 0);
    mul(self, other) = { Self(self.0 + other.0, self.1 + other.1) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountedAdd(pub i64);
monoid! {
    CountedAdd;
    one = CountedAdd(0);
    mul(self, other) = { Self(self.0 + other.0) };
}
impl Act<CountedSum> for CountedAdd {
    fn act(&self, other: CountedSum) -> CountedSum {
        CountedSum(other.0 + self.0 * other.1 as i64, other.1)
    }
}

impl RangedAddSegmentTreeSum {
    pub fn from(v: Vec<i64>) -> Self {
        let t = LazySegmentTree::from(v.iter().map(|&x| CountedSum(x, 1)).collect());
        Self { t }
    }
    pub fn to_vec(&mut self) -> Vec<i64> {
        self.t.to_vec().iter().map(|cs| cs.0).collect()
    }
    pub fn add(&mut self, range: std::ops::Range<usize>, x: i64) {
        self.t.update(range, CountedAdd(x));
    }
    pub fn sum(&mut self, range: std::ops::Range<usize>) -> i64 {
        self.t.product(range).0
    }
}

#[cfg(test)]
mod test_ranged_add_segment_tree_sum {
    use crate::sequence::tree::ranged_add_segment_tree_sum::*;
    #[test]
    fn from_zero() {
        let mut t = RangedAddSegmentTreeSum::from(vec![0; 5]); // [0, 0, 0, 0, 0]
        t.t.debug();
        assert_eq!(t.sum(1..3), 0);
        assert_eq!(t.sum(0..4), 0);
        t.add(1..3, 2); // [0, 2, 2, 0, 0]
        t.t.debug();
        assert_eq!(t.sum(0..5), 4);
        assert_eq!(t.sum(0..0), 0);
        assert_eq!(t.sum(0..1), 0);
        assert_eq!(t.sum(0..2), 2);
        assert_eq!(t.sum(1..2), 2);
        assert_eq!(t.sum(1..3), 4);
        t.add(2..4, -1); // [0, 2, 1, -1, 0]
        t.t.debug();
        assert_eq!(t.sum(0..5), 2);
        assert_eq!(t.sum(1..3), 3);
        assert_eq!(t.sum(2..5), 0);
    }
}
