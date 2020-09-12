/// Geometry2D - Definition of Circle
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
        let eps = 1e-6;
        self.center == other.center && (self.radius - other.radius).abs() < eps
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
}
