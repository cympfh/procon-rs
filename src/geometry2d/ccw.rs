/// Geometry2D - CCW (直線と点の関係)
use crate::geometry2d::line::*;
use crate::geometry2d::point::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CCW {
    FRONT,
    BACK,
    RIGHT,
    LEFT,
    ON,
}

pub fn ccw(l: LineSegment, p: Point) -> CCW {
    use CCW::*;
    fn equal(x: f64, y: f64) -> bool {
        (x - y).abs() < 0.00001
    }
    let dif = p - l.0;
    let dir = l.1 - l.0;
    if equal(0.0, dir.0) {
        if equal(0.0, dif.0) {
            let k = dif.1 / dir.1;
            if k > 1.0 {
                FRONT
            } else if k < 0.0 {
                BACK
            } else {
                ON
            }
        } else if dir.det(&dif) > 0.0 {
            LEFT
        } else {
            RIGHT
        }
    } else {
        let k = dif.0 / dir.0;
        if equal(dir.1 * k, dif.1) {
            if k > 1.0 {
                FRONT
            } else if k < 0.0 {
                BACK
            } else {
                ON
            }
        } else {
            if dir.det(&dif) > 0.0 {
                LEFT
            } else {
                RIGHT
            }
        }
    }
}

#[cfg(test)]
mod test_ccw {
    use crate::geometry2d::ccw::*;
    use CCW::*;

    #[test]
    fn it_works() {
        let p = Point(1.0, 2.0);
        let q = Point(2.0, 0.0);
        let l = LineSegment::new(p, q);

        assert_eq!(ccw(l, p), ON);
        assert_eq!(ccw(l, q), ON);
        assert_eq!(ccw(l, Point(1.5, 1.0)), ON);
        assert_eq!(ccw(l, Point(3.0, -2.0)), FRONT);
        assert_eq!(ccw(l, Point(0.0, 4.0)), BACK);
        assert_eq!(ccw(l, Point(0.0, 0.0)), RIGHT);
        assert_eq!(ccw(l, Point(10.0, 0.0)), LEFT);
    }
}
