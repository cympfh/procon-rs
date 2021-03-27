/// Geometry2D - Definition of Point
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }
    pub fn zero() -> Point {
        Point(0.0, 0.0)
    }
    pub fn norm(&self) -> f64 {
        (*self * *self).sqrt()
    }
    pub fn det(&self, other: &Point) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }
    pub fn arg(&self) -> f64 {
        let x = self.0 / self.norm();
        let y = self.1 / self.norm();
        y.atan2(x)
    }
    pub fn distance(&self, other: &Point) -> f64 {
        (*self - *other).norm()
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        let eps = 1e-6;
        (self.0 - other.0).abs() < eps && (self.1 - other.1).abs() < eps
    }
    fn ne(&self, other: &Point) -> bool {
        !(self == other)
    }
}
impl Eq for Point {}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point(-self.0, -self.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        self + (-other)
    }
}
// scalar multiplication
impl std::ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point(self * other.0, self * other.1)
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point(other * self.0, other * self.1)
    }
}
// inner-product
impl std::ops::Mul<Point> for Point {
    type Output = f64;
    fn mul(self, other: Point) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
}
impl std::ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, other: f64) -> Point {
        Point(self.0 / other, self.1 / other)
    }
}

#[cfg(test)]
mod test_point {
    use crate::geometry2d::point::*;

    #[test]
    fn norm() {
        assert_eq!(Point(1.0, 0.0).norm(), 1.0);
        assert_eq!(Point(3.0, 4.0).norm(), 5.0);
    }

    #[test]
    fn distance() {
        assert_eq!(Point(0.0, 0.0).distance(&Point(3.0, 0.0)), 3.0);
        assert_eq!(Point(0.0, 0.0).distance(&Point(3.0, 4.0)), 5.0);
    }

    #[test]
    fn arithmetic() {
        assert_eq!(Point(1.0, 2.0) * -1.0, Point(-1.0, -2.0));
        assert_eq!(Point(1.0, 2.0) * Point(1.0, -1.0), -1.0);
        assert_eq!(Point(1.0, 2.0) / 2.0, Point(0.5, 1.0));
        assert_eq!(Point(1.0, 2.0) + Point::zero(), Point(1.0, 2.0));
        assert_eq!(Point(1.0, 2.0) - Point(0.0, 1.0), Point(1.0, 1.0));
    }
}
