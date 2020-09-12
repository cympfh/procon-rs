/// Geometry2D - Intersection of 2 Circles
use crate::geometry2d::circle::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleIntersectionRelation {
    EQUAL,
    SUB,
    SUP,                 // one contains the other
    INTERSECTION(usize), // intersection with `n` points
}

impl Circle {
    pub fn intersection(&self, other: &Circle) -> CircleIntersectionRelation {
        use CircleIntersectionRelation::*;
        let d = (self.center - other.center).norm();
        let eps = 1e-6;
        if self == other {
            EQUAL
        } else if d + self.radius < other.radius {
            SUB
        } else if self.radius > d + other.radius {
            SUP
        } else if d < self.radius + other.radius {
            INTERSECTION(2)
        } else if d <= self.radius + other.radius + eps {
            INTERSECTION(1)
        } else {
            INTERSECTION(0)
        }
    }
}

#[cfg(test)]
mod test_circle_intersection {
    use crate::geometry2d::circle_intersection::*;
    use crate::geometry2d::point::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Circle::new(Point::zero(), 1.0).intersection(&Circle::new(Point::zero(), 1.1)),
            CircleIntersectionRelation::SUB
        );
        assert_eq!(
            Circle::new(Point::zero(), 1.0).intersection(&Circle::new(Point::zero(), 0.9)),
            CircleIntersectionRelation::SUP
        );
        assert_eq!(
            Circle::new(Point::zero(), 1.0).intersection(&Circle::new(Point(1.0, 0.0), 1.0)),
            CircleIntersectionRelation::INTERSECTION(2)
        );
        assert_eq!(
            Circle::new(Point::zero(), 1.0).intersection(&Circle::new(Point(2.0, 0.0), 1.0)),
            CircleIntersectionRelation::INTERSECTION(1)
        );
        assert_eq!(
            Circle::new(Point::zero(), 1.0).intersection(&Circle::new(Point(2.1, 0.0), 1.0)),
            CircleIntersectionRelation::INTERSECTION(0)
        );
    }
}
