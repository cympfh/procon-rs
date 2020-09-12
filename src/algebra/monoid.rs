/// Algebra - Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self>
where
    Self: std::marker::Sized,
{
    fn unit() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub i64);
impl std::ops::Mul for Sum {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
impl Monoid for Sum {
    fn unit() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prod(pub i64);
impl std::ops::Mul for Prod {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}
impl Monoid for Prod {
    fn unit() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod test_monoid {
    use crate::algebra::monoid::*;

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
