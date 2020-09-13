/// Sequence - Lazy Segment Tree - Ranged RMQ
use crate::algebra::act_assign::*;
use crate::algebra::monoid_minmax::*;
use crate::sequence::tree::lazy_segment_tree::*;

pub type RangedRMaxQ<X> = LazySegmentTree<MaxInt<X>, Assign<MaxInt<X>>>;
pub type RangedRMinQ<X> = LazySegmentTree<MinInt<X>, Assign<MinInt<X>>>;

#[cfg(test)]
mod test_ratio {
    use crate::sequence::tree::ranged_rmq::*;

    #[test]
    fn it_works() {
        let v: Vec<i64> = vec![1, 2, 3, 4];
        let mut rmq = RangedRMaxQ::from(v.iter().map(|&x| MaxInt::Val(x)).collect());
        assert_eq!(rmq.product(1..3), MaxInt::Val(3));

        rmq.update(1..3, Assign::Some(MaxInt::Val(1))); // [1,1,1,4]
        assert_eq!(rmq.product(1..3), MaxInt::Val(1));
        assert_eq!(rmq.product(2..4), MaxInt::Val(4));
    }
}
