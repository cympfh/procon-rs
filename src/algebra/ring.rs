/// Algebra - Ring (+, -, 0, *, /, 1)
use crate::algebra::group::*;
pub trait Ring: Group + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> {
    fn one() -> Self;
}
macro_rules! define_ring {
    ($t:ty, $x:expr) => {
        impl Ring for $t {
            fn one() -> Self {
                $x
            }
        }
    };
}
define_ring!(i32, 1);
define_ring!(i64, 1);
define_ring!(i128, 1);
define_ring!(f32, 1.0);
define_ring!(f64, 1.0);
