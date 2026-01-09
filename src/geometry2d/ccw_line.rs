/// Geometry2D - CCW (直線と点の関係)
use crate::geometry2d::line::*;
use crate::geometry2d::point::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CCW {
    Right,
    Left,
    Online,
}

pub fn ccw(a: Point, b: Point, c: Point) -> i32 {
    let u = b - a;
    let v = c - a;
    let d = u.det(&v);
    if d > 0.0 {
        1
    } else if d < 0.0 {
        -1
    } else {
        0
    }
}

pub fn ccw_line(l: Line, p: Point) -> CCW {
    let d = ccw(l.0, l.1, p);
    if d > 0 {
        CCW::Left
    } else if d < 0 {
        CCW::Right
    } else {
        CCW::Online
    }
}

#[cfg(test)]
mod test_ccw_line {
    use crate::geometry2d::ccw_line::*;

    #[test]
    fn it_works() {
        let l = Line(Point(0.0, 0.0), Point(1.0, 1.0));
        assert_eq!(ccw_line(l, Point(1.0, 0.0)), CCW::Right);
        assert_eq!(ccw_line(l, Point(0.0, 1.0)), CCW::Left);
        assert_eq!(ccw_line(l, Point(2.0, 2.0)), CCW::Online);
    }
}
