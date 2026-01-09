/// Geometry2D - CCW (線分と点の関係)
use crate::geometry2d::line::*;
use crate::geometry2d::point::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CCWLineSegment {
    Right,
    Left,
    Onsegment,
    Front,
    Back,
}

pub fn ccw_linesegment(l: LineSegment, p: Point) -> CCWLineSegment {
    use CCWLineSegment::*;
    let u = l.1 - l.0;
    let v = p - l.0;
    let ccw = u.det(&v);
    let eps = 1e-9;
    if ccw > eps {
        Left
    } else if ccw < -eps {
        Right
    } else {
        if u * v < -eps {
            Back
        } else if u.norm() < v.norm() {
            Front
        } else {
            Onsegment
        }
    }
}

#[cfg(test)]
mod test_ccw {
    use crate::geometry2d::ccw_linesegment::*;
    use CCWLineSegment::*;

    #[test]
    fn it_works() {
        let p = Point(1.0, 2.0);
        let q = Point(2.0, 0.0);
        let l = LineSegment::new(p, q);

        assert_eq!(ccw_linesegment(l, p), Onsegment);
        assert_eq!(ccw_linesegment(l, q), Onsegment);
        assert_eq!(ccw_linesegment(l, Point(1.5, 1.0)), Onsegment);
        assert_eq!(ccw_linesegment(l, Point(3.0, -2.0)), Front);
        assert_eq!(ccw_linesegment(l, Point(0.0, 4.0)), Back);
        assert_eq!(ccw_linesegment(l, Point(0.0, 0.0)), Right);
        assert_eq!(ccw_linesegment(l, Point(10.0, 0.0)), Left);
    }
}
