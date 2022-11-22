/// Geometry2D - Definition of Circle
use crate::almost_equal; // IGNORE
use crate::geometry2d::point::*;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        assert!(radius >= 0.0);
        Self { center, radius }
    }
    pub fn area(&self) -> f64 {
        self.radius.powi(2) * std::f64::consts::PI
    }
}
impl std::cmp::PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && almost_equal!(self.radius, other.radius)
    }
}

/// Circle vs Circle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleIntersection {
    Equal,
    Sub,              // self is contained by the other
    Sup,              // self contains the other
    Intersect(usize), // intersection with `n` points
    Outer,            // no join
}

impl Circle {
    pub fn intersection(&self, other: &Circle) -> CircleIntersection {
        use CircleIntersection::*;
        let d = (self.center - other.center).norm();
        let d2 = (self.center - other.center).quadrance();
        let r2 = (self.radius + other.radius).powi(2);
        let l2 = (self.radius - other.radius).powi(2);
        if self == other {
            Equal
        } else if almost_equal!(d2, r2) || almost_equal!(d2, l2) {
            Intersect(1)
        } else if d + self.radius < other.radius {
            Sub
        } else if self.radius > d + other.radius {
            Sup
        } else if d < self.radius + other.radius {
            Intersect(2)
        } else {
            Outer
        }
    }
}

#[cfg(test)]
mod test_circle {
    use crate::geometry2d::circle::*;

    #[test]
    fn area() {
        assert_eq!(Circle::new(Point(0.0, 0.0), 0.0).area(), 0.0);
        assert_eq!(
            Circle::new(Point(0.0, 0.0), 1.0).area(),
            std::f64::consts::PI
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Circle::new(Point(0.0, 0.0), 1.0),
            Circle::new(Point(0.0, 0.0), 1.0)
        );
        assert_ne!(
            Circle::new(Point(0.0, 0.0), 1.0),
            Circle::new(Point(0.0, 0.1), 1.0)
        );
        assert_ne!(
            Circle::new(Point(0.0, 0.0), 1.0),
            Circle::new(Point(0.0, 0.0), 1.1)
        );
    }

    #[test]
    fn test_intersection() {
        let c = Circle::new(Point(0., 0.), 5.);
        let d = Circle::new(Point(7., 0.), 2.);
        assert_eq!(c.intersection(&d), CircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), CircleIntersection::Intersect(1));

        let c = Circle::new(Point(0., 0.), 5.);
        let d = Circle::new(Point(7., 0.), 1.);
        assert_eq!(c.intersection(&d), CircleIntersection::Outer);
        assert_eq!(d.intersection(&c), CircleIntersection::Outer);

        let c = Circle::new(Point(0., 0.), 6.);
        let d = Circle::new(Point(7., 0.), 2.);
        assert_eq!(c.intersection(&d), CircleIntersection::Intersect(2));
        assert_eq!(d.intersection(&c), CircleIntersection::Intersect(2));

        let c = Circle::new(Point(0., 0.), 10.);
        let d = Circle::new(Point(7., 0.), 2.);
        assert_eq!(c.intersection(&d), CircleIntersection::Sup);
        assert_eq!(d.intersection(&c), CircleIntersection::Sub);

        let c = Circle::new(Point(-4., 3.), 5.);
        let d = Circle::new(Point(4., -3.), 5.);
        assert_eq!(c.intersection(&d), CircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), CircleIntersection::Intersect(1));

        let c = Circle::new(Point(0., 10.), 5.);
        let d = Circle::new(Point(0., 12.), 3.);
        assert_eq!(c.intersection(&d), CircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), CircleIntersection::Intersect(1));

        let c = Circle::new(Point(0., 10.), 5.);
        let d = Circle::new(Point(0., 12.), 4.);
        assert_eq!(c.intersection(&d), CircleIntersection::Intersect(2));
        assert_eq!(d.intersection(&c), CircleIntersection::Intersect(2));

        let c = Circle::new(Point(0., 10.), 5.);
        let d = Circle::new(Point(0., 12.), 2.);
        assert_eq!(c.intersection(&d), CircleIntersection::Sup);
        assert_eq!(d.intersection(&c), CircleIntersection::Sub);
    }
}
