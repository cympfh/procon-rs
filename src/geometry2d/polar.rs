/// Geometry - Polar Coordinates System
use crate::geometry2d::point::*;

#[derive(Debug, Clone, Copy)]
pub struct Polar {
    theta: f64,
    r: f64,
}
impl Polar {
    pub const PI: f64 = std::f64::consts::PI;
    pub fn new(theta: f64, r: f64) -> Self {
        let mut t = theta;
        let r = if r >= 0.0 {
            r
        } else {
            t += Self::PI;
            -r
        };
        t %= Self::PI * 2.0;
        if t < 0.0 {
            t += Self::PI * 2.0;
        }
        Polar { theta: t, r }
    }
    pub fn distance(&self, other: &Self) -> f64 {
        (self.r.powi(2) + other.r.powi(2)
            - 2.0 * self.r * other.r * (self.theta - other.theta).abs().cos())
        .sqrt()
    }
    pub fn to_xy(&self) -> Point {
        Point(self.r * self.theta.cos(), self.r * self.theta.sin())
    }
    pub fn from_xy(x: &Point) -> Self {
        let r = x.norm();
        let theta = x.arg();
        Self { theta, r }
    }
}

#[cfg(test)]
mod test_line {
    use crate::geometry2d::polar::*;

    #[test]
    fn distance() {
        assert_eq!(
            Polar::new(0.0, 1.0).distance(&Polar::new(Polar::PI, 2.0)),
            3.0
        );
    }

    #[test]
    fn to_xy() {
        assert_eq!(Polar::new(0.0, 1.0).to_xy(), Point(1.0, 0.0));
        assert_eq!(Polar::new(Polar::PI / 2.0, 1.0).to_xy(), Point(0.0, 1.0));
        assert_eq!(Polar::new(Polar::PI, 1.0).to_xy(), Point(-1.0, 0.0));
    }

    #[test]
    fn from_xy() {
        assert_eq!(Polar::from_xy(&Point(1.0, 0.0)).to_xy(), Point(1.0, 0.0));
        assert_eq!(Polar::from_xy(&Point(1.0, -2.0)).to_xy(), Point(1.0, -2.0));
    }
}
