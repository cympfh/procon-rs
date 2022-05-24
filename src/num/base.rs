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
pub trait Num: Nat + std::ops::Neg<Output = Self> {}

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
        one = $one:expr
        $(;)*
    ) => {
        def_nat!($ty; zero = $zero; one = $one);
        impl Num for $ty {}
    };
}

def_nat! { usize; zero = 0; one = 1 }
def_nat! { u64; zero = 0; one = 1 }
def_num! { i64; zero = 0; one = 1 }
def_num! { i128; zero = 0; one = 1 }
def_num! { Float; zero = Float(0.0); one = Float(1.0); }

#[cfg(test)]
mod test_num {
    use crate::num::Num;
    #[test]
    fn test_num() {
        fn work<X: Num>(x: X, y: X) -> X {
            (x + y) * (y - X::one())
        }
        assert_eq!(work(2_i64, 3), 10);
    }
}
