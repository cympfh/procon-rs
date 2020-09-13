pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait Num:
    Copy
    + Eq
    + Ord
    + Zero
    + One
    + std::marker::Sized
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
{
}
pub trait Int: Num {}
pub trait Nat: Num {}

macro_rules! define_zero_one {
    ($ty:ty, $zero:expr, $one:expr) => {
        impl Zero for $ty {
            fn zero() -> Self {
                $zero
            }
        }
        impl One for $ty {
            fn one() -> Self {
                $one
            }
        }
    };
}

define_zero_one!(usize, 0, 1);
define_zero_one!(u32, 0, 1);
define_zero_one!(u64, 0, 1);
define_zero_one!(u128, 0, 1);
define_zero_one!(i32, 0, 1);
define_zero_one!(i64, 0, 1);
define_zero_one!(i128, 0, 1);
define_zero_one!(f32, 0.0, 1.0);
define_zero_one!(f64, 0.0, 1.0);

impl Num for usize {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Nat for usize {}
impl Nat for u32 {}
impl Nat for u64 {}
impl Nat for u128 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
