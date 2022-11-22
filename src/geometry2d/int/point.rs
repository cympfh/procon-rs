/// Geometry2D/Int - Definition of Point

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntPoint(pub i128, pub i128);

impl IntPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self(x, y)
    }
    pub fn zero() -> Self {
        Self(0, 0)
    }
    pub fn cross(&self, other: IntPoint) -> i128 {
        self.0 * other.1 - self.1 * other.0
    }
    pub fn quadrance(&self) -> i128 {
        self.0.pow(2) + self.1.pow(2)
    }
}

impl std::ops::Add<IntPoint> for IntPoint {
    type Output = Self;
    fn add(self, other: IntPoint) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub<IntPoint> for IntPoint {
    type Output = Self;
    fn sub(self, other: IntPoint) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl std::ops::Mul<i128> for IntPoint {
    type Output = Self;
    fn mul(self, k: i128) -> Self {
        Self(self.0 * k, self.1 * k)
    }
}
impl std::ops::Div<i128> for IntPoint {
    type Output = Self;
    fn div(self, k: i128) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

#[cfg(test)]
mod test_int_point {

    use crate::geometry2d::int::point::*;

    #[test]
    fn test_add() {
        assert_eq!(IntPoint(1, 2) + IntPoint(2, 1), IntPoint(3, 3));
    }

    #[test]
    fn test_sub() {
        assert_eq!(IntPoint(1, 2) - IntPoint(2, 1), IntPoint(-1, 1));
    }

    #[test]
    fn test_mul() {
        assert_eq!(IntPoint(1, 2) * -2, IntPoint(-2, -4));
    }

    #[test]
    fn test_div() {
        assert_eq!(IntPoint(1, 2) / -2, IntPoint(0, -1));
    }

    #[test]
    fn test_cross() {
        assert_eq!(IntPoint(1, 1).cross(IntPoint(2, 2)), 0);
        assert_eq!(IntPoint(1, 0).cross(IntPoint(2, 2)), 2);
        assert_eq!(IntPoint(0, 1).cross(IntPoint(2, 2)), -2);
        assert_eq!(IntPoint(2, 1).cross(IntPoint(-1, 2)), 5);
    }
}
