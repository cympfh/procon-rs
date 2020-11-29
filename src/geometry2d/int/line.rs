/// Geometry2D/Int - Definition of Line
use crate::geometry2d::int::point::*;

pub struct IntLine(pub IntPoint, pub IntPoint);

impl IntLine {
    pub fn contains(&self, p: IntPoint) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

#[macro_export]
macro_rules! iline {
    ($x0:expr, $y0:expr; $x1:expr, $y1:expr) => {
        IntLine(IntPoint($x0, $y0), IntPoint($x1, $y1))
    };
    ($a:expr; $b:expr) => {
        IntLine($a, $b)
    };
}

#[cfg(test)]
mod test_int_line {

    use crate::geometry2d::int::line::*;

    #[test]
    fn test_sub() {
        assert!(iline!(0, 0; 1, 2).contains(IntPoint(2, 4)));
        assert!(iline!(0, 0; 1, 2).contains(IntPoint(-3, -6)));
        assert!(iline!(0, 0; 1, 2).contains(IntPoint(0, 0)));
        assert!(iline!(1, 1; 1, 2).contains(IntPoint(1, 31)));
    }
}
