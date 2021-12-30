/// Sequence - Range Maximum/Minimum Query
use crate::algebra::monoid_max::*;
use crate::algebra::monoid_min::*;
use crate::sequence::tree::segment_tree::*;

pub type RMaxQ<X> = SegmentTree<MaxInt<X>>;
pub type RMinQ<X> = SegmentTree<MinInt<X>>;

#[cfg(test)]
mod test_rmq {
    use crate::sequence::tree::rmq::*;

    #[test]
    fn from_new() {
        let mut x = RMaxQ::<i64>::new(10);
        let mut y = RMinQ::<i64>::new(10);

        x.update(1, MaxInt::Val(1));
        y.update(1, MinInt::Val(1));

        assert_eq!(x.product(0..1), MaxInt::Minimal);
        assert_eq!(y.product(0..1), MinInt::Maximal);
        assert_eq!(x.product(0..2), MaxInt::Val(1));
        assert_eq!(y.product(0..2), MinInt::Val(1));
        assert_eq!(x.product(0..3), MaxInt::Val(1));
        assert_eq!(y.product(0..3), MinInt::Val(1));

        x.update(2, MaxInt::Val(2));
        y.update(2, MinInt::Val(2));

        assert_eq!(x.product(0..1), MaxInt::Minimal);
        assert_eq!(y.product(0..1), MinInt::Maximal);
        assert_eq!(x.product(0..2), MaxInt::Val(1));
        assert_eq!(y.product(0..2), MinInt::Val(1));
        assert_eq!(x.product(0..3), MaxInt::Val(2));
        assert_eq!(y.product(0..3), MinInt::Val(1));
    }

    #[test]
    fn from_an_array() {
        let v: Vec<i64> = vec![1, 2, 3, 4];
        let x = RMaxQ::from(v.iter().map(|&x| MaxInt::Val(x)).collect());
        assert_eq!(x.product(1..4), MaxInt::Val(4));
        assert_eq!(x.product(2..4), MaxInt::Val(4));
        assert_eq!(x.product(2..3), MaxInt::Val(3));
        assert_eq!(x[0], MaxInt::Val(1));
        assert_eq!(x[1], MaxInt::Val(2));
        assert_eq!(x[2], MaxInt::Val(3));
        assert_eq!(x[3], MaxInt::Val(4));
    }
}
