/// Algebra - Monoid - MaxInt
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE

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
    MaxInt<X> where [X:Ord];
    one = MaxInt::Minimal;
    mul(self, other) = {
        if self > other {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod test_monoid_maxint {
    use crate::algebra::monoid_max::*;
    #[test]
    fn test_max() {
        assert_eq!(MaxInt::Val(2).unwrap(), 2);
        assert_eq!(MaxInt::Val(1) * MaxInt::Val(2), MaxInt::Val(2));
        assert_eq!(MaxInt::Val(2) * MaxInt::Val(1), MaxInt::Val(2));
        assert_eq!(MaxInt::Val(2) * MaxInt::Minimal, MaxInt::Val(2));
        assert_eq!(MaxInt::Minimal * MaxInt::Val(1), MaxInt::Val(1));
    }
}
