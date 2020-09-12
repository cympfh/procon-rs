/// Geometry2D - Definition of Line
use crate::geometry2d::point::*;

#[derive(Debug, Clone, Copy)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        assert!(x != y);
        Self(x, y)
    }
    pub fn distance_from(&self, p: Point) -> f64 {
        let u = p - self.0;
        let v = self.1 - self.0;
        (u.det(&v) / v.norm()).abs()
    }
}
impl std::cmp::PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        let a = Point::zero();
        let b = Point(1.0, 0.0);
        let c = Point(0.0, 1.0);
        let eps = 1e-6;
        for p in &[a, b, c] {
            if (self.distance_from(*p) - other.distance_from(*p)).abs() > eps {
                return false;
            }
        }
        true
    }
}
impl std::cmp::Eq for Line {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment(pub Point, pub Point);

impl LineSegment {
    pub fn new(x: Point, y: Point) -> Self {
        assert!(x != y);
        Self(x, y)
    }
    pub fn to_line(&self) -> Line {
        Line(self.0, self.1)
    }
}
impl std::ops::Neg for LineSegment {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.1, self.0)
    }
}

#[macro_export]
macro_rules! line {
    ($x0:expr, $y0:expr; $x1:expr, $y1:expr) => {
        Line::new(Point($x0, $y0), Point($x1, $y1))
    };
    ($x0:expr, $y0:expr => $x1:expr, $y1:expr) => {
        LineSegment::new(Point($x0, $y0), Point($x1, $y1))
    };
    ($a:expr; $b:expr) => {
        Line::new($a, $b)
    };
    ($a:expr => $b:expr) => {
        LineSegment::new($a, $b)
    };
}

#[cfg(test)]
mod test_line {
    use crate::geometry2d::line::*;

    #[test]
    fn use_macro() {
        assert_eq!(
            line!(0.0, 0.0; 1.0, 1.0),
            Line::new(Point(0.0, 0.0), Point(1.0, 1.0))
        );
        assert_eq!(
            line!(0.0, 0.0 => 1.0, 1.0),
            LineSegment::new(Point(0.0, 0.0), Point(1.0, 1.0))
        );
        let p = Point(1.0, 2.0);
        let q = Point(2.0, -1.0);
        assert_eq!(line!(p; q), Line::new(p, q));
        assert_eq!(line!(p => q), LineSegment::new(p, q));
    }

    #[test]
    fn line_equality() {
        assert_eq!(line!(0.0, 0.0; 1.0, 1.0), line!(2.0, 2.0; -1.0, -1.0),);
        assert_eq!(line!(0.0, 0.0; 1.0, 1.0), line!(1.0, 1.0; 2.0, 2.0),);
        assert_ne!(line!(0.0, 0.0; 1.0, 1.0), line!(1.0, 1.0; 2.0, 2.01),);
    }

    #[test]
    fn line_segment_equality() {
        assert_eq!(line!(0.0, 0.0 => 1.0, 1.0), line!(0.0, 0.0 => 1.0, 1.0),);
        assert_ne!(line!(0.0, 0.0 => 1.0, 1.0), line!(0.0, 0.0 => 1.0, 1.01),);
        assert_ne!(line!(0.0, 1.0 => 1.0, 0.0), line!(1.0, 0.0 => 0.0, 1.0),);
    }

    #[test]
    fn arithmetic() {
        assert_eq!(line!(0.0, 1.0 => 1.0, 0.0), -line!(1.0, 0.0 => 0.0, 1.0),);
    }
}
