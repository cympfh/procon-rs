/// Algebra - Ratio Numbers
use crate::algebra::group::*;
use crate::algebra::ring::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ratio(i64, i64); // Normalized (numerator / denominator)

impl Ratio {
    fn new(a: i64, b: i64) -> Self {
        let x = Self::gcd(a.abs(), b.abs());
        let sign = if b > 0 { 1 } else { -1 };
        Ratio(sign * a / x, sign * b / x)
    }
    fn from(x: i64) -> Self {
        Ratio(x, 1)
    }
    fn inv(&self) -> Self {
        if self.0 > 0 {
            Ratio(self.1, self.0)
        } else if self.0 < 0 {
            Ratio(-self.1, -self.0)
        } else {
            Ratio(1, 0) // Infinity!
        }
    }
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }
}
impl std::ops::Neg for Ratio {
    type Output = Self;
    fn neg(self) -> Self {
        Ratio(-self.0, self.1)
    }
}
impl std::ops::Add for Ratio {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let num = Self::lcm(self.1, other.1);
        Ratio::new(self.0 * num / self.1 + other.0 * num / other.1, num)
    }
}
impl std::ops::Sub for Ratio {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}
impl std::ops::Mul for Ratio {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0, self.1 * other.1)
    }
}
impl std::ops::Mul<i64> for Ratio {
    type Output = Self;
    fn mul(self, z: i64) -> Self {
        Self(self.0 * z, self.1)
    }
}
impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.0 * other.1, self.1 * other.0)
    }
}
impl std::iter::Sum for Ratio {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Ratio::zero(), std::ops::Add::add)
    }
}
impl Group for Ratio {
    fn zero() -> Self {
        Ratio(0, 1)
    }
}
impl Ring for Ratio {
    fn one() -> Self {
        Ratio(1, 1)
    }
}
impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.0 * other.1;
        let right = other.0 * self.1;
        left.partial_cmp(&right)
    }
}
impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[cfg(test)]
mod test_ratio {
    use crate::algebra::ratio::*;
    #[test]
    fn it_works() {
        assert_eq!(Ratio(1, 2) + Ratio(1, 2), Ratio::one());
        assert_eq!(Ratio(1, 2) - Ratio(1, 2), Ratio::zero());
        assert_eq!(Ratio::one() * Ratio::zero(), Ratio::zero());
        assert_eq!(Ratio::zero() * Ratio::one(), Ratio::zero());
        assert_eq!(Ratio::zero() * 2, Ratio::zero());
        assert_eq!(Ratio::one() * -1, -Ratio::one());
        assert_eq!(Ratio::one() / Ratio::from(2), Ratio(1, 2));
    }
}
