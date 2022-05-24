/// Algebra - Monoid - MinInt
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE

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
    MinInt<X> where [X:Ord];
    one = MinInt::Maximal;
    mul(self, other) = {
        if self < other {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod test_monoid_minint {
    use crate::algebra::monoid_min::*;
    #[test]
    fn test_min() {
        assert_eq!(MinInt::Val(2).unwrap(), 2);
        assert_eq!(MinInt::Val(1) * MinInt::Val(2), MinInt::Val(1));
        assert_eq!(MinInt::Val(2) * MinInt::Val(1), MinInt::Val(1));
        assert_eq!(MinInt::Val(2) * MinInt::Maximal, MinInt::Val(2));
        assert_eq!(MinInt::Maximal * MinInt::Val(1), MinInt::Val(1));
    }
}
