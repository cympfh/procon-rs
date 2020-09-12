/// Algebra - Group (+, -, 0)
pub trait Group:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
{
    fn zero() -> Self;
}
macro_rules! define_group {
    ($t:ty, $x:expr) => {
        impl Group for $t {
            fn zero() -> Self {
                $x
            }
        }
    };
}
define_group!(i32, 0);
define_group!(i64, 0);
define_group!(i128, 0);
define_group!(f32, 0.0);
define_group!(f64, 0.0);

#[cfg(test)]
mod test_group {
    use crate::algebra::group::Group;
    #[test]
    fn it_works() {
        assert_eq!(i32::zero(), 0);
        assert_eq!(i32::zero() + 3, 3);
        assert_eq!(3 + i32::zero(), 3);
        assert_eq!(1 + 2 + 3, 6);
        assert_eq!(2 + (-2), i32::zero());
    }
}
