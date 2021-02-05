/// Algebra - Def of Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self>
where
    Self: std::marker::Sized,
{
    fn unit() -> Self;
}

#[macro_export]
macro_rules! monoid {
    (
        [ $( $params:tt )* ]
        for $type:ty;
        unit = $unit:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> Monoid for $type {
            fn unit() -> Self { $unit }
        }
    };
    (
        for $type:ty;
        unit = $unit:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { [] for $type; unit = $unit; mul($self, $y) = $code; }
    };
}
