/// Geometry2D/Int - Definition of Circle
use crate::geometry2d::int::point::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntCircle {
    pub center: IntPoint,
    pub radius: i128,
}
impl IntCircle {
    pub fn new(center: IntPoint, radius: i128) -> Self {
        assert!(radius >= 0);
        Self { center, radius }
    }
}

/// IntCircle vs IntPoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntCircleInlude {
    Inner,
    Outer,
    On,
}

impl IntCircle {
    pub fn include(&self, p: IntPoint) -> IntCircleInlude {
        let d2 = (self.center - p).quadrance();
        let r2 = self.radius.pow(2);
        if d2 == r2 {
            IntCircleInlude::On
        } else if d2 > r2 {
            IntCircleInlude::Outer
        } else {
            IntCircleInlude::Inner
        }
    }
}

/// IntCircle vs IntCircle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntCircleIntersection {
    Equal,
    Sub,              // self is contained by the other
    Sup,              // self contains the other
    Intersect(usize), // intersection with `n` points
    Outer,            // no join
}

impl IntCircle {
    pub fn intersection(&self, other: &IntCircle) -> IntCircleIntersection {
        use IntCircleIntersection::*;
        let d2 = (self.center - other.center).quadrance();
        let r2 = (self.radius + other.radius).pow(2);
        let l2 = (self.radius - other.radius).pow(2);
        if self == other {
            Equal
        } else if d2 == r2 || d2 == l2 {
            Intersect(1)
        } else if d2 < l2 && self.radius < other.radius {
            Sub
        } else if d2 < l2 && self.radius > other.radius {
            Sup
        } else if d2 < r2 {
            Intersect(2)
        } else {
            Outer
        }
    }
}

#[cfg(test)]
mod test_int_circle {

    use crate::geometry2d::int::circle::*;

    #[test]
    fn test_include() {
        let c = IntCircle::new(IntPoint(0, 0), 5);
        assert_eq!(c.include(IntPoint(0, 0)), IntCircleInlude::Inner);
        assert_eq!(c.include(IntPoint(0, 1)), IntCircleInlude::Inner);
        assert_eq!(c.include(IntPoint(-5, 0)), IntCircleInlude::On);
        assert_eq!(c.include(IntPoint(3, 4)), IntCircleInlude::On);
        assert_eq!(c.include(IntPoint(-3, -4)), IntCircleInlude::On);
        assert_eq!(c.include(IntPoint(4, 4)), IntCircleInlude::Outer);
        assert_eq!(c.include(IntPoint(5, 1)), IntCircleInlude::Outer);

        let c = IntCircle::new(IntPoint(-10, 1), 5);
        assert_eq!(c.include(IntPoint(-7, 5)), IntCircleInlude::On);
        assert_eq!(c.include(IntPoint(-13, -3)), IntCircleInlude::On);
    }

    #[test]
    fn test_intersection() {
        let c = IntCircle::new(IntPoint(0, 0), 5);
        let d = IntCircle::new(IntPoint(7, 0), 2);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), IntCircleIntersection::Intersect(1));

        let c = IntCircle::new(IntPoint(0, 0), 5);
        let d = IntCircle::new(IntPoint(7, 0), 1);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Outer);
        assert_eq!(d.intersection(&c), IntCircleIntersection::Outer);

        let c = IntCircle::new(IntPoint(0, 0), 6);
        let d = IntCircle::new(IntPoint(7, 0), 2);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Intersect(2));
        assert_eq!(d.intersection(&c), IntCircleIntersection::Intersect(2));

        let c = IntCircle::new(IntPoint(0, 0), 10);
        let d = IntCircle::new(IntPoint(7, 0), 2);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Sup);
        assert_eq!(d.intersection(&c), IntCircleIntersection::Sub);

        let c = IntCircle::new(IntPoint(-4, 3), 5);
        let d = IntCircle::new(IntPoint(4, -3), 5);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), IntCircleIntersection::Intersect(1));

        let c = IntCircle::new(IntPoint(0, 10), 5);
        let d = IntCircle::new(IntPoint(0, 12), 3);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Intersect(1));
        assert_eq!(d.intersection(&c), IntCircleIntersection::Intersect(1));

        let c = IntCircle::new(IntPoint(0, 10), 5);
        let d = IntCircle::new(IntPoint(0, 12), 4);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Intersect(2));
        assert_eq!(d.intersection(&c), IntCircleIntersection::Intersect(2));

        let c = IntCircle::new(IntPoint(0, 10), 5);
        let d = IntCircle::new(IntPoint(0, 12), 2);
        assert_eq!(c.intersection(&d), IntCircleIntersection::Sup);
        assert_eq!(d.intersection(&c), IntCircleIntersection::Sub);
    }
}
