/// Def of Float Numbers
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::algebra::ring::*;
use crate::{agroup, monoid, ring}; // IGNORE

#[derive(Debug, Clone, Copy)]
pub struct Float(pub f64);

impl Float {
    pub fn abs(&self) -> Self {
        Float(self.0.abs())
    }
    pub fn sin(&self) -> Self {
        Float(self.0.sin())
    }
    pub fn cos(&self) -> Self {
        Float(self.0.cos())
    }
    pub fn tan(&self) -> Self {
        Float(self.0.tan())
    }
}

agroup! {
    Float;
    zero = Float(0.0);
    add(self, other) = { Float(self.0 + other.0) };
    neg(self) = { Float(-self.0) };
}
monoid! {
    Float;
    one = Float(1.0);
    mul(self, other) = { Float(self.0 * other.0) };
}
ring! {
    Float;
    div(self, other) = { Float(self.0 / other.0) };
}
impl std::ops::Rem for Float {
    type Output = Self;
    fn rem(self, modulo: Self) -> Float {
        Float(self.0 % modulo.0)
    }
}
impl std::ops::RemAssign for Float {
    fn rem_assign(&mut self, modulo: Self) {
        self.0 %= modulo.0;
    }
}
impl std::cmp::PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        let allow = 1e-6;
        (self.0 - other.0).abs() < allow
            || (self.0 - other.0).abs() / (self.0 + other.0) * 2.0 < allow
    }
}
impl std::cmp::Eq for Float {}
impl std::cmp::PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[cfg(test)]
mod test_float {
    use crate::num::float::Float;
    #[test]
    fn test_comparable() {
        let mut x = vec![Float(1.0), Float(0.0), Float(-1.0)];
        x.sort();
        assert_eq!(x, vec![Float(-1.0), Float(0.0), Float(1.0),]);
    }
    #[test]
    fn test_equality() {
        use crate::num::base::*;
        let x = Float(3.14);
        let y = Float(3.141);
        let z = Float(3.140000001);
        assert_ne!(x, y);
        assert_eq!(x, z);
        assert_eq!(Float(0.0), Float::zero());
        assert_eq!(Float(2e-7), Float::zero());
    }
}
