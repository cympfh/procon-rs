/// Algebra - Complex Number
use crate::algebra::ring::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Complex<X>(X, X);

impl<X: Ring + Copy> Complex<X> {
    fn zero() -> Complex<X> {
        Complex(X::zero(), X::zero())
    }
    fn unit() -> Complex<X> {
        Complex(X::one(), X::zero())
    }
    fn i() -> Complex<X> {
        Complex(X::zero(), X::one())
    }
    fn real(&self) -> X {
        self.0
    }
    fn imag(&self) -> X {
        self.1
    }
    fn scalar_mul(&self, x: X) -> Self {
        Complex(self.0 * x, self.1 * x)
    }
    fn scalar_div(&self, x: X) -> Self {
        Complex(self.0 / x, self.1 / x)
    }
}
impl<X: Ring> std::ops::Add<Complex<X>> for Complex<X> {
    type Output = Self;
    fn add(self, right: Self) -> Self {
        Complex(self.0 + right.0, self.1 + right.1)
    }
}
impl<X: Ring> std::ops::Neg for Complex<X> {
    type Output = Self;
    fn neg(self) -> Self {
        Complex(-self.0, -self.1)
    }
}
impl<X: Ring> std::ops::Sub<Complex<X>> for Complex<X> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}
impl<X: Ring + Copy> std::ops::Mul<Complex<X>> for Complex<X> {
    type Output = Self;
    fn mul(self, right: Self) -> Self {
        Complex(
            self.0 * right.0 - self.1 * right.1,
            self.0 * right.1 + self.1 * right.0,
        )
    }
}
impl<X: Ring + Copy> std::ops::Mul<X> for Complex<X> {
    type Output = Self;
    fn mul(self, x: X) -> Self::Output {
        self.scalar_mul(x)
    }
}
impl<X: Ring + Copy> std::ops::Div<X> for Complex<X> {
    type Output = Self;
    fn div(self, x: X) -> Self::Output {
        self.scalar_div(x)
    }
}

#[cfg(test)]
mod test_complex {
    use crate::algebra::complex::*;
    #[test]
    fn it_works() {
        let x = Complex::<f64>::unit();
        assert_eq!(x, Complex(1.0, 0.0));
        assert_eq!(Complex::<i32>::unit() * Complex::i(), Complex::i());
        assert_eq!(Complex::<i32>::unit().real(), 1);
        assert_eq!(Complex::<i32>::unit().imag(), 0);
        assert_eq!(Complex::<i32>::i().real(), 0);
        assert_eq!(Complex::<i32>::i().imag(), 1);

        assert_eq!(-Complex::<i32>::unit(), Complex(-1, 0));
        assert_eq!(Complex::unit() - Complex::<i32>::unit(), Complex::zero());
        assert_eq!(Complex::unit() + Complex::unit(), Complex(2, 0));

        assert_eq!(Complex::i() * Complex::i(), Complex(-1, 0));

        assert_eq!(Complex(1_i32, -2) * -1, Complex(-1, 2));
        assert_eq!(Complex::<f64>(2.0, 4.0) / 2.0, Complex(1.0, 2.0));
    }
}
