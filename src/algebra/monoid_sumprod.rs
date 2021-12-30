/// Algebra - Def of Monoid (i64, +), (i64, *)
use crate::algebra::monoid::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumMonoid(pub i64);
monoid! {
    SumMonoid;
    one = SumMonoid(0);
    mul(self, other) = {
        Self(self.0 + other.0)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProdMonoid(pub i64);
monoid! {
    ProdMonoid;
    one = ProdMonoid(1);
    mul(self, other) = {
        Self(self.0 * other.0)
    };
}

#[cfg(test)]
mod test_monoid_sumprod {
    use crate::algebra::monoid_sumprod::{ProdMonoid, SumMonoid};

    #[test]
    fn sum() {
        assert_eq!(SumMonoid(0) * SumMonoid(3), SumMonoid(3));
        assert_eq!(SumMonoid(3) * SumMonoid(0), SumMonoid(3));
        assert_eq!(SumMonoid(1) * SumMonoid(2) * SumMonoid(4), SumMonoid(7));
    }

    #[test]
    fn prod() {
        assert_eq!(ProdMonoid(1) * ProdMonoid(3), ProdMonoid(3));
        assert_eq!(ProdMonoid(3) * ProdMonoid(1), ProdMonoid(3));
        assert_eq!(
            ProdMonoid(2) * ProdMonoid(3) * ProdMonoid(5),
            ProdMonoid(30)
        );
    }
}
