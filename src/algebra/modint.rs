/// Algebra - ModInt (Zp)

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ModInt(i64, i64); // (residual, modulo)
impl ModInt {
    fn new(residual: i64, modulo: i64) -> ModInt {
        if residual >= modulo {
            ModInt(residual % modulo, modulo)
        } else if residual < 0 {
            ModInt((residual % modulo) + modulo, modulo)
        } else {
            ModInt(residual, modulo)
        }
    }
    fn unwrap(self) -> i64 {
        self.0
    }
    fn inv(self) -> Self {
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
    fn pow(self, n: i64) -> Self {
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
impl std::ops::Neg for ModInt {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            return self;
        }
        ModInt(self.1 - self.0, self.1)
    }
}
impl std::ops::Add<i64> for ModInt {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        ModInt::new(self.0 + other, self.1)
    }
}
impl std::ops::Add for ModInt {
    type Output = Self;
    fn add(self, other: ModInt) -> Self {
        self + other.0
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
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: ModInt) {
        *self += other.0;
    }
}
impl std::ops::Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, other: i64) -> Self {
        ModInt::new(self.0 - other, self.1)
    }
}
impl std::ops::Sub for ModInt {
    type Output = Self;
    fn sub(self, other: ModInt) -> Self {
        self - other.0
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
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: ModInt) {
        *self -= other.0;
    }
}
impl std::ops::Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        ModInt::new(self.0 * other, self.1)
    }
}
impl std::ops::Mul for ModInt {
    type Output = Self;
    fn mul(self, other: ModInt) -> Self {
        self * other.0
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
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: ModInt) {
        *self *= other.0;
    }
}
impl std::ops::Div for ModInt {
    type Output = Self;
    fn div(self, other: ModInt) -> Self {
        self * other.inv()
    }
}
impl std::ops::Div<i64> for ModInt {
    type Output = Self;
    fn div(self, other: i64) -> Self {
        self / ModInt(other, self.1)
    }
}
impl std::ops::Div<ModInt> for i64 {
    type Output = ModInt;
    fn div(self, other: ModInt) -> ModInt {
        other.inv() * self
    }
}
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: ModInt) {
        self.0 = (self.clone() / other).0;
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

    const MOD: i64 = 10007;

    #[test]
    fn it_works() {
        assert_eq!(ModInt(1, MOD) + ModInt(1, MOD), ModInt(2, MOD));
        assert_eq!(ModInt(1, MOD) - ModInt(1, MOD), ModInt(0, MOD));
        assert_eq!((-ModInt(1, MOD)) + ModInt(1, MOD), ModInt(0, MOD));
        assert_eq!(
            (ModInt(1, MOD) / ModInt(2, MOD)) * ModInt(2, MOD),
            ModInt(1, MOD)
        );
        {
            let mut m = ModInt(0, MOD);
            m -= 1;
            assert_eq!(m.unwrap(), MOD - 1);
            m = m * 2 + 2;
            assert_eq!(m.unwrap(), 0);
            m /= 2;
            assert_eq!(m.unwrap(), 0);
        }
    }
}
