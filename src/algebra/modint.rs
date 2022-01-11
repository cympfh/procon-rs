/// Algebra - ModInt (Z/pZ)
use crate::agroup; // IGNORE
use crate::algebra::field::*;
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::algebra::ring::*;
use crate::mint; // IGNORE
use crate::monoid; // IGNORE
use crate::ring; // IGNORE
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModInt(pub i64, pub i64); // (residual, modulo)

pub const MOD_1000000007: i64 = 1_000_000_007;
pub const MOD_998244353: i64 = 998_244_353;
#[macro_export]
macro_rules! mint {
    ($x:expr) => {
        ModInt::new($x, MOD_1000000007)
    };
}

impl ModInt {
    pub fn new(residual: i64, modulo: i64) -> ModInt {
        if residual >= modulo {
            ModInt(residual % modulo, modulo)
        } else if residual < 0 {
            ModInt((residual % modulo) + modulo, modulo)
        } else {
            ModInt(residual, modulo)
        }
    }
    pub fn unwrap(self) -> i64 {
        self.0
    }
    pub fn inv(self) -> Self {
        fn exgcd(r0: i64, a0: i64, b0: i64, r: i64, a: i64, b: i64) -> (i64, i64, i64) {
            if r > 0 {
                exgcd(r, a, b, r0 % r, a0 - r0 / r * a, b0 - r0 / r * b)
            } else {
                (a0, b0, r0)
            }
        }
        let (a, _, r) = exgcd(self.0, 1, 0, self.1, 0, 1);
        if r != 1 {
            panic!("{:?} has no inverse!", self);
        }
        ModInt(((a % self.1) + self.1) % self.1, self.1)
    }
    pub fn pow(self, n: i64) -> Self {
        if n < 0 {
            self.pow(-n).inv()
        } else if n == 0 {
            ModInt(1, self.1)
        } else if n == 1 {
            self
        } else {
            let mut x = (self * self).pow(n / 2);
            if n % 2 == 1 {
                x *= self
            }
            x
        }
    }
}
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
agroup! {
    ModInt;
    zero = mint!(0);
    add(self, other) = { ModInt::new(self.0 + other.0, self.1) };
    neg(self) = {
        if self.0 == 0 {
            self
        } else {
            ModInt(self.1 - self.0, self.1)
        }
    };
}
monoid! {
    ModInt;
    one = mint!(1);
    mul(self, other) = { ModInt::new(self.0 * other.0, self.1) };
}
ring! {
    ModInt;
    div(self, other) = { self * other.inv() };
}
impl Field for ModInt {}

impl std::ops::Add<i64> for ModInt {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        ModInt::new(self.0 + other, self.1)
    }
}
impl std::ops::Add<ModInt> for i64 {
    type Output = ModInt;
    fn add(self, other: ModInt) -> ModInt {
        other + self
    }
}
impl std::ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 + other, self.1).0;
    }
}
impl std::ops::Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, other: i64) -> Self {
        ModInt::new(self.0 - other, self.1)
    }
}
impl std::ops::Sub<ModInt> for i64 {
    type Output = ModInt;
    fn sub(self, other: ModInt) -> ModInt {
        ModInt::new(self - other.0, other.1)
    }
}
impl std::ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 - other, self.1).0;
    }
}
impl std::ops::Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        ModInt::new(self.0 * other, self.1)
    }
}
impl std::ops::Mul<ModInt> for i64 {
    type Output = ModInt;
    fn mul(self, other: ModInt) -> ModInt {
        other * self
    }
}
impl std::ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 * other, self.1).0;
    }
}
impl std::ops::Div<i64> for ModInt {
    type Output = Self;
    fn div(self, other: i64) -> Self {
        self / ModInt::new(other, self.1)
    }
}
impl std::ops::Div<ModInt> for i64 {
    type Output = ModInt;
    fn div(self, other: ModInt) -> ModInt {
        other.inv() * self
    }
}
impl std::ops::DivAssign<i64> for ModInt {
    fn div_assign(&mut self, other: i64) {
        *self /= ModInt(other, self.1);
    }
}

#[cfg(test)]
mod test_modint {
    use crate::algebra::modint::*;

    #[test]
    fn it_works() {
        assert_eq!(mint!(1) + mint!(1), mint!(2));
        assert_eq!(mint!(1) + mint!(-1), mint!(0));
        assert_eq!(mint!(1) - 1, mint!(0));
        assert_eq!(mint!(1) + mint!(-3), mint!(-2));
        assert_eq!(mint!(1) - mint!(-3), mint!(4));
        assert_eq!(mint!(-3) + 1, mint!(-2));
        assert_eq!(-mint!(1), mint!(-1));
        assert_eq!(mint!(-1) * 2, mint!(-2));
        assert_eq!((mint!(1) / -3) * 6, mint!(-2));
    }

    #[test]
    fn test_pow() {
        assert_eq!(mint!(2).pow(10), mint!(1024));
        assert_eq!(mint!(-2).pow(10), mint!(1024));
        assert_eq!(mint!(-2).pow(11), mint!(-2048));
    }

    #[test]
    fn test_mut() {
        let mut m = mint!(0);

        m -= 1;
        assert_eq!(m, mint!(-1));
        assert_eq!(m.unwrap(), MOD_1000000007 - 1);

        m *= 2;
        assert_eq!(m, mint!(-2));

        m += 2;
        assert_eq!(m, mint!(0));
        assert_eq!(m.unwrap(), 0);

        m += 1;
        m /= 3; // m = 1/3
        assert_eq!(m + m + m, mint!(1));
    }
}
