/// Sequence - Lazy Segment Tree - Ranged Add/RMQ
use crate::agroup; // IGNORE
use crate::algebra::act_add::*;
use crate::algebra::group_additive::*;
use crate::algebra::monoid_max::*;
use crate::algebra::monoid_min::*;
use crate::sequence::tree::lazy_segment_tree::*;

pub type RangedAddRMaxQ<X> = LazySegmentTree<MaxInt<X>, Add<MaxInt<X>>>;
agroup! {
    MaxInt<X> where [X: AGroup + Copy];
    zero = MaxInt::Val(X::zero());
    add(self, other) = {
        match (self, other) {
            (MaxInt::Val(x), MaxInt::Val(y)) => MaxInt::Val(x + y),
            _ => MaxInt::Minimal,
        }
    };
    neg(self) = {
        match self {
            MaxInt::Val(x) => MaxInt::Val(-x),
            _ => panic!("Cannot negate MaxInt::Minimal"),
        }
    };
}

pub type RangedAddRMinQ<X> = LazySegmentTree<MinInt<X>, Add<MinInt<X>>>;
agroup! {
    MinInt<X> where [X: AGroup + Copy];
    zero = MinInt::Val(X::zero());
    add(self, other) = {
        match (self, other) {
            (MinInt::Val(x), MinInt::Val(y)) => MinInt::Val(x + y),
            _ => MinInt::Maximal,
        }
    };
    neg(self) = {
        match self {
            MinInt::Val(x) => MinInt::Val(-x),
            _ => panic!("Cannot negate MinInt::Maximal"),
        }
    };
}

#[cfg(test)]
mod test_ranged_add_rmq {
    use crate::sequence::tree::ranged_add_rmq::*;

    #[test]
    fn test_max() {
        let v: Vec<i64> = vec![1, 2, 3, 4];
        let mut rmq = RangedAddRMaxQ::from(v.iter().map(|&x| MaxInt::Val(x)).collect());
        assert_eq!(rmq.product(1..3), MaxInt::Val(3));

        rmq.update(1..3, Add(MaxInt::Val(2))); // [1, 4, 5, 4]
        assert_eq!(rmq.product(0..2), MaxInt::Val(4));
        assert_eq!(rmq.product(1..3), MaxInt::Val(5));
        assert_eq!(rmq.product(2..4), MaxInt::Val(5));
    }

    #[test]
    fn test_min() {
        let v: Vec<i64> = vec![1, 2, 3, 4];
        let mut rmq = RangedAddRMinQ::from(v.iter().map(|&x| MinInt::Val(x)).collect());
        assert_eq!(rmq.product(1..3), MinInt::Val(2));

        rmq.update(1..3, Add(MinInt::Val(2))); // [1, 4, 5, 4]
        assert_eq!(rmq.product(0..2), MinInt::Val(1));
        assert_eq!(rmq.product(1..3), MinInt::Val(4));
        assert_eq!(rmq.product(2..4), MinInt::Val(4));
        assert_eq!(rmq.product(2..3), MinInt::Val(5));
    }
}
