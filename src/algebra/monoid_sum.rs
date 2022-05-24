/// Algebra - Def of Monoid (i64, +)
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub i64);
monoid! {
    Sum;
    one = Sum(0);
    mul(self, other) = {
        Self(self.0 + other.0)
    };
}

#[cfg(test)]
mod test_monoid_sum {
    use crate::algebra::monoid_sum::*;

    #[test]
    fn sum() {
        assert_eq!(Sum(0) * Sum(3), Sum(3));
        assert_eq!(Sum(3) * Sum(0), Sum(3));
        assert_eq!(Sum(1) * Sum(2) * Sum(4), Sum(7));
        assert_eq!(Sum(2), Sum(2) * Sum::one());
        assert_eq!(Sum(2), Sum::one() * Sum(2));
    }
}
