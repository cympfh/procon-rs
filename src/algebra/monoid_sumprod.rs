/// Algebra - Def of Monoid (i64, +), (i64, *)
use crate::algebra::monoid::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub i64);
monoid! {
    for Sum;
    unit = Sum(0);
    mul(self, other) = {
        Self(self.0 + other.0)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prod(pub i64);
monoid! {
    for Prod;
    unit = Prod(1);
    mul(self, other) = {
        Self(self.0 * other.0)
    };
}

#[cfg(test)]
mod test_monoid {
    use crate::algebra::monoid_sumprod::{Prod, Sum};

    #[test]
    fn sum() {
        assert_eq!(Sum(0) * Sum(3), Sum(3));
        assert_eq!(Sum(3) * Sum(0), Sum(3));
        assert_eq!(Sum(1) * Sum(2) * Sum(4), Sum(7));
    }

    #[test]
    fn prod() {
        assert_eq!(Prod(1) * Prod(3), Prod(3));
        assert_eq!(Prod(3) * Prod(1), Prod(3));
        assert_eq!(Prod(2) * Prod(3) * Prod(5), Prod(30));
    }
}
