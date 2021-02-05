/// Algebra - Monoid - MinInt, MaxInt
use crate::algebra::monoid::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxInt<X> {
    Minimal,
    Val(X),
}
impl<X> MaxInt<X> {
    pub fn unwrap(self) -> X {
        match self {
            Self::Val(x) => x,
            _ => panic!(),
        }
    }
}
monoid! {
    [X:Ord] for MaxInt<X>;
    unit = MaxInt::Minimal;
    mul(self, other) = {
        if self > other {
            self
        } else {
            other
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MinInt<X> {
    Val(X),
    Maximal,
}
impl<X> MinInt<X> {
    pub fn unwrap(self) -> X {
        match self {
            Self::Val(x) => x,
            _ => panic!(),
        }
    }
}
monoid! {
    [X:Ord] for MinInt<X>;
    unit = MinInt::Maximal;
    mul(self, other) = {
        if self < other {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod test_monoid_rmq {
    use crate::algebra::monoid_minmax::*;
    #[test]
    fn test_max() {
        assert_eq!(MaxInt::Val(2).unwrap(), 2);
        assert_eq!(MaxInt::Val(1) * MaxInt::Val(2), MaxInt::Val(2));
        assert_eq!(MaxInt::Val(2) * MaxInt::Val(1), MaxInt::Val(2));
        assert_eq!(MaxInt::Val(2) * MaxInt::Minimal, MaxInt::Val(2));
        assert_eq!(MaxInt::Minimal * MaxInt::Val(1), MaxInt::Val(1));
    }
    #[test]
    fn test_min() {
        assert_eq!(MinInt::Val(2).unwrap(), 2);
        assert_eq!(MinInt::Val(1) * MinInt::Val(2), MinInt::Val(1));
        assert_eq!(MinInt::Val(2) * MinInt::Val(1), MinInt::Val(1));
        assert_eq!(MinInt::Val(2) * MinInt::Maximal, MinInt::Val(2));
        assert_eq!(MinInt::Maximal * MinInt::Val(1), MinInt::Val(1));
    }
}
