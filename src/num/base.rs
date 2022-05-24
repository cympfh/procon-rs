/// Nat, Num trait
use crate::num::float::*;

pub trait Nat:
    Copy
    + Eq
    + Ord
    + std::marker::Sized
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
{
    fn zero() -> Self;
    fn one() -> Self;
}
pub trait Num: Nat + std::ops::Neg<Output = Self> {
    fn almosteq(self, other: Self) -> bool;
}

#[macro_export]
macro_rules! def_nat {
    (
        $ty:ty;
        zero = $zero:expr;
        one = $one:expr
        $(;)*
    ) => {
        impl Nat for $ty {
            fn zero() -> Self {
                $zero
            }
            fn one() -> Self {
                $one
            }
        }
    };
}

#[macro_export]
macro_rules! def_num {
    (
        $ty:ty;
        zero = $zero:expr;
        one = $one:expr;
        almosteq($self:ident, $other:ident) = $almosteq:block
        $(;)*
    ) => {
        def_nat!($ty; zero = $zero; one = $one);
        impl Num for $ty {
            fn almosteq($self, $other: Self) -> bool { $almosteq }
        }
    };
}

def_nat! { usize; zero = 0; one = 1 }
def_nat! { u64; zero = 0; one = 1 }
def_num! { i64; zero = 0; one = 1; almosteq(self, other) = { self == other } }
def_num! { i128; zero = 0; one = 1; almosteq(self, other) = { self == other } }
def_num! { Float; zero = Float(0.0); one = Float(1.0); almosteq(self, other) = {
        let allow = 1e-6;
        (self.0 - other.0).abs() < allow || (self.0 - other.0).abs() / (self.0 + other.0) * 2.0 < allow
    };
}

#[cfg(test)]
mod test_num {
    use crate::num::float::Float;
    use crate::num::Num;
    #[test]
    fn test_num() {
        fn work<X: Num>(x: X, y: X) -> X {
            (x + y) * (y - X::one())
        }
        assert_eq!(work(2_i64, 3), 10);
    }
    #[test]
    fn test_equal() {
        assert!((32_i64).almosteq(32_i64));
        assert!(!(32_i64).almosteq(31_i64));
        let x = Float(3.14);
        let y = x + Float(2e-7);
        assert!(x.almosteq(y));
        let z = Float(3.141);
        assert!(!x.almosteq(z));
    }
}
