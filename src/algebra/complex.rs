/// Algebra - Complex Number
use crate::agroup; // IGNORE
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::algebra::ring::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<X>(pub X, pub X);

impl<X: Copy> Complex<X> {
    pub fn real(&self) -> X {
        self.0
    }
    pub fn imag(&self) -> X {
        self.1
    }
}
agroup! {
    Complex<X> where [X: Copy + AGroup];
    zero = Complex(X::zero(), X::zero());
    add(self, other) = { Complex(self.0 + other.0, self.1 + other.1) };
    neg(self) =  { Complex(-self.0, -self.1) };
}
monoid! {
    Complex<X> where [X: AGroup + Monoid + Copy];
    one = Complex(X::one(), X::zero());
    mul(self,other) = {
        Complex(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}
impl<X: AGroup + Monoid + Copy> Ring for Complex<X> {}
impl<X: AGroup + Monoid + Copy> Complex<X> {
    pub fn i() -> Complex<X> {
        Complex(X::zero(), X::one())
    }
}
impl<X: std::ops::Add<Output = X>> std::ops::Add<X> for Complex<X> {
    type Output = Self;
    fn add(self, x: X) -> Self {
        Complex(self.0 + x, self.1)
    }
}
impl<X: std::ops::Sub<Output = X>> std::ops::Sub<X> for Complex<X> {
    type Output = Self;
    fn sub(self, x: X) -> Self {
        Complex(self.0 - x, self.1)
    }
}
impl<X: std::ops::Mul<Output = X> + Copy> std::ops::Mul<X> for Complex<X> {
    type Output = Self;
    fn mul(self, x: X) -> Self::Output {
        Self(self.0 * x, self.1 * x)
    }
}
impl<X: Ring + std::ops::Div<Output = X> + Copy> std::ops::Div<X> for Complex<X> {
    type Output = Self;
    fn div(self, x: X) -> Self::Output {
        Self(self.0 / x, self.1 / x)
    }
}

#[cfg(test)]
mod test_complex {
    use crate::algebra::complex::*;
    #[test]
    fn test_intish_complex() {
        let x = Complex::<i64>::one();
        assert_eq!(x, Complex(1, 0));
        assert_eq!(x + 2, Complex(3, 0));
        assert_eq!(x - 2, Complex(-1, 0));
        assert_eq!(x + Complex(0, -1), Complex(1, -1));
        assert_eq!(x * 2, Complex(2, 0));
        assert_eq!(x * Complex(-1, 1), Complex(-1, 1));
    }
    #[test]
    fn test_intish_realish() {
        assert_eq!(Complex(1.0, 2.0) / 2.0, Complex(0.5, 1.0));
    }
    #[test]
    fn test_assign() {
        let mut x = Complex(0, 0);
        x += Complex(0, 1);
        x *= Complex(1, 1);
        x -= Complex(0, 1);
        assert_eq!(
            x,
            ((Complex(0, 0) + Complex(0, 1)) * Complex(1, 1)) - Complex(0, 1)
        );
    }
}
