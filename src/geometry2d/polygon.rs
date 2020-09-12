/// Geometry2D - Definition of Polygon
use crate::geometry2d::point::*;

#[derive(Debug, Clone)]
pub struct Polygon(Vec<Point>);

impl Polygon {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::ops::Index<usize> for Polygon {
    type Output = Point;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl Polygon {
    pub fn area(&self) -> f64 {
        (1..self.len() - 1)
            .map(|i| {
                let u = self[i] - self[0];
                let v = self[i + 1] - self[0];
                u.det(&v).abs()
            })
            .sum::<f64>()
            / 2.0
    }
}

#[macro_export]
macro_rules! poly {
    ($($x:expr),+) => {{
        let v = vec![$($x),+];
        Polygon(v)
    }};
    ($($x:expr),+ ,) => (poly!($($x),+))
}

#[cfg(test)]
mod test_line {
    use crate::geometry2d::polygon::*;

    #[test]
    fn area() {
        let tri = poly![Point(0.0, 0.0), Point(1.0, 0.0), Point(1.0, 1.0),];
        assert_eq!(tri.area(), 0.5);
        let sq = poly![
            Point(0.0, 0.0),
            Point(1.0, 0.0),
            Point(1.0, 1.0),
            Point(0.0, 1.0),
        ];
        assert_eq!(sq.area(), 1.0);
    }
}
