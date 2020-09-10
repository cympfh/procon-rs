/// Algebra - Act
pub trait Act<X> {
    fn act(&self, x: X) -> X;
}
