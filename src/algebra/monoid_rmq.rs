/// Algebra - RMQ Monoid
use crate::algebra::monoid::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxInt<X> {
    Minimal,
    Val(X),
}
impl<X> MaxInt<X> {
    fn unwrap(self) -> X {
        if let Self::Val(x) = self {
            x
        } else {
            panic!()
        }
    }
}
impl<X: Ord> std::ops::Mul for MaxInt<X> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
}
impl<X: Ord + Copy> Monoid for MaxInt<X> {
    fn unit() -> Self {
        MaxInt::Minimal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MinInt<X> {
    Val(X),
    Maximal,
}
impl<X> MinInt<X> {
    fn unwrap(self) -> X {
        if let Self::Val(x) = self {
            x
        } else {
            panic!();
        }
    }
}
impl<X: Ord> std::ops::Mul for MinInt<X> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}
impl<X: Ord + Copy> Monoid for MinInt<X> {
    fn unit() -> Self {
        MinInt::Maximal
    }
}

#[cfg(test)]
mod test_monoid_rmq {
    use crate::algebra::monoid_rmq::*;
    #[test]
    fn test_max() {
        assert_eq!(MaxInt::Val(2).unwrap(), 2);
        assert_eq!(MaxInt::Val(2) * MaxInt::Val(1), MaxInt::Val(2));
        assert_eq!(MaxInt::Val(2) * MaxInt::Minimal, MaxInt::Val(2));
        assert_eq!(MaxInt::Minimal * MaxInt::Val(1), MaxInt::Val(1));
    }
    #[test]
    fn test_min() {
        assert_eq!(MinInt::Val(2).unwrap(), 2);
        assert_eq!(MinInt::Val(2) * MinInt::Val(1), MinInt::Val(1));
        assert_eq!(MinInt::Val(2) * MinInt::Maximal, MinInt::Val(2));
        assert_eq!(MinInt::Maximal * MinInt::Val(1), MinInt::Val(1));
    }
}
