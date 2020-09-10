/// Algebra - Monoid
pub trait Monoid: std::ops::Mul<Output = Self> + Clone + Copy {
    fn unit() -> Self;
}
macro_rules! define_monoid {
    ($type:ty,$unit:expr) => {
        impl Monoid for $type {
            fn unit() -> Self {
                $unit
            }
        }
    };
}
define_monoid!(usize, 0);
define_monoid!(u32, 0);
define_monoid!(u64, 0);
define_monoid!(u128, 0);
define_monoid!(i32, 0);
define_monoid!(i64, 0);
define_monoid!(i128, 0);
define_monoid!(f32, 0.0);
define_monoid!(f64, 0.0);
