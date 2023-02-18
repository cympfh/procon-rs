/// Algebra - Ratio Numbers
use crate::agroup; // IGNORE
use crate::algebra::field::*;
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::algebra::ring::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio(i128, i128); // Normalized (numerator / denominator)

impl Ratio {
    pub fn new(a: i128, b: i128) -> Self {
        Ratio(a, b).normalize()
    }
    fn normalize(&mut self) -> Self {
        let g = Self::gcd(self.0.abs(), self.1.abs());
        self.0 /= g;
        self.1 /= g;
        if self.1 < 0 {
            self.0 *= -1;
            self.1 *= -1;
        }
        *self
    }
    pub fn from(x: i128) -> Self {
        Ratio(x, 1)
    }
    pub fn inv(&self) -> Self {
        if self.0 > 0 {
            Ratio(self.1, self.0)
        } else if self.0 < 0 {
            Ratio(-self.1, -self.0)
        } else {
            Ratio(1, 0) //Infinity
        }
    }
    fn gcd(a: i128, b: i128) -> i128 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn lcm(a: i128, b: i128) -> i128 {
        a / Self::gcd(a, b) * b
    }
}
agroup! {
    Ratio;
    zero = Ratio(0, 1);
    add(self, other) = {
        let num = Self::lcm(self.1, other.1);
        Ratio::new(self.0 * num / self.1 + other.0 * num / other.1, num)
    };
    neg(self) = {
        Ratio(-self.0, self.1)
    };
}
monoid! {
    Ratio;
    one = Ratio(1, 1);
    mul(self, other) = { Self::new(self.0 * other.0, self.1 * other.1) }
}
impl Ring for Ratio {}

impl std::ops::Add<i128> for Ratio {
    type Output = Self;
    fn add(self, z: i128) -> Self {
        Ratio::new(self.0 + self.1 * z, self.1)
    }
}
impl std::ops::AddAssign<i128> for Ratio {
    fn add_assign(&mut self, z: i128) {
        self.0 += self.1 * z;
        self.normalize();
    }
}
impl std::ops::Sub<i128> for Ratio {
    type Output = Self;
    fn sub(self, z: i128) -> Self {
        self + (-z)
    }
}
impl std::ops::SubAssign<i128> for Ratio {
    fn sub_assign(&mut self, z: i128) {
        *self += -z;
    }
}
impl std::ops::Mul<i128> for Ratio {
    type Output = Self;
    fn mul(self, z: i128) -> Self {
        Self::new(self.0 * z, self.1)
    }
}
impl std::ops::MulAssign<i128> for Ratio {
    fn mul_assign(&mut self, z: i128) {
        self.0 *= z;
        self.normalize();
    }
}
impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.0 * other.1, self.1 * other.0)
    }
}
impl Field for Ratio {}
impl std::ops::Div<i128> for Ratio {
    type Output = Self;
    fn div(self, other: i128) -> Self {
        Self::new(self.0, self.1 * other)
    }
}
impl std::ops::DivAssign<i128> for Ratio {
    fn div_assign(&mut self, z: i128) {
        self.1 *= z;
        self.normalize();
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

#[macro_export]
macro_rules! r {
    ($x:expr, $y:expr) => {
        Ratio::new($x, $y)
    };
}

#[cfg(test)]
mod test_ratio {
    use crate::algebra::ratio::*;

    #[test]
    fn it_works() {
        assert_eq!(r!(1, 2) + r!(1, 2), Ratio(1, 1));
        assert_eq!(r!(1, 3) + r!(2, 3), Ratio(1, 1));
        assert_eq!(r!(1, 3) + r!(1, 3) + r!(1, 3), Ratio(1, 1));
        assert_eq!(r!(1, 3) + 2, r!(7, 3));
        assert_eq!(r!(1, 3) - r!(1, 3), Ratio(0, 1));
        assert_eq!(r!(1, 3) - r!(2, 3), Ratio(-1, 3));
        assert_eq!(r!(1, 2) - r!(1, 3), r!(1, 6));
        assert_eq!(r!(1, 3) - r!(0, 2), Ratio(1, 3));
        assert_eq!(r!(1, 1) * r!(1, 1), Ratio(1, 1));
        assert_eq!(r!(1, 3) * r!(2, 3), Ratio(2, 9));
        assert_eq!(r!(2, 3) * -1, Ratio(-2, 3));
        assert_eq!(r!(2, 3) / -1, Ratio(-2, 3));
        assert_eq!(r!(2, 3) / r!(2, 1), r!(1, 3));
        assert_eq!(
            Ratio::zero() * 2 + Ratio::one() / Ratio::from(2),
            Ratio(1, 2)
        );
    }

    #[test]
    fn test_inv() {
        assert_eq!(r!(1, 1).inv(), r!(1, 1));
        assert_eq!(r!(1, 2).inv(), r!(2, 1));
        assert_eq!(r!(1, -2).inv(), r!(-2, 1));
        // 1/0 = Inf
        assert_eq!(r!(0, 1).inv(), r!(1, 0));
        assert_eq!(r!(0, 2).inv(), r!(1, 0));
        assert_eq!(r!(0, -3).inv(), r!(1, 0));
        // 1/Inf = 0
        assert_eq!(r!(1, 0).inv(), r!(0, 1));
    }

    #[test]
    fn test_zero() {
        assert_eq!(r!(0, 1), Ratio(0, 1));
        assert_eq!(r!(0, 2), Ratio(0, 1));
        assert_eq!(r!(0, -3), Ratio(0, 1));
    }

    #[test]
    fn test_mut() {
        let mut r = r!(1, 2);
        r *= 2;
        assert_eq!(r, Ratio(1, 1));

        r -= 1;
        assert_eq!(r, Ratio(0, 1));

        r += r!(1, 3);
        assert_eq!(r, Ratio(1, 3));

        r -= r!(1, 6);
        assert_eq!(r, Ratio(1, 6));

        r = r.inv();
        assert_eq!(r, Ratio(6, 1));
    }
}
