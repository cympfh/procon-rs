/// Geometry2D - 最近点対 (Closest Pair of Points)
/// 分割統治法による O(n log² n) の実装
use crate::geometry2d::point::*;

pub fn closest_pair(ps: &Vec<Point>) -> (Point, Point) {
    let mut sorted_ps = ps.clone();
    sorted_ps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    closest_pair_rec(&sorted_ps)
}

fn closest_pair_rec(ps: &[Point]) -> (Point, Point) {
    let n = ps.len();

    // 小さい場合は全探索
    if n < 5 {
        let mut min_dist = f64::INFINITY;
        let mut result = (ps[0], ps[1]);
        for i in 0..n {
            for j in 0..i {
                let d = ps[i].distance(&ps[j]);
                if d < min_dist {
                    min_dist = d;
                    result = (ps[i], ps[j]);
                }
            }
        }
        return result;
    }

    // 左右に分割して再帰的に解く
    let mid = n / 2;
    let left_pair = closest_pair_rec(&ps[..mid]);
    let right_pair = closest_pair_rec(&ps[mid..]);

    // 左右でより近い方を選ぶ
    let left_dist = left_pair.0.distance(&left_pair.1);
    let right_dist = right_pair.0.distance(&right_pair.1);
    let (mut best_pair, mut min_dist) = if left_dist < right_dist {
        (left_pair, left_dist)
    } else {
        (right_pair, right_dist)
    };

    // 境界付近をチェック
    let mid_x = ps[mid].0;
    let lower_x = mid_x - min_dist;
    let upper_x = mid_x + min_dist;

    // 境界付近の点を集める
    let mut strip: Vec<Point> = ps
        .iter()
        .filter(|p| lower_x <= p.0 && p.0 <= upper_x)
        .copied()
        .collect();

    // y座標でソート
    strip.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // 各点について近傍(最大7点)をチェック
    for i in 0..strip.len() {
        for j in (i + 1)..strip.len().min(i + 8) {
            let d = strip[i].distance(&strip[j]);
            if d < min_dist {
                min_dist = d;
                best_pair = (strip[i], strip[j]);
            }
        }
    }

    best_pair
}

#[cfg(test)]
mod test_closest_pair {
    use crate::geometry2d::closest_pair::*;
    use crate::geometry2d::point::Point;

    #[test]
    fn it_works() {
        let ps = vec![
            Point(0.0, 0.0),
            Point(1.0, 1.0),
            Point(2.0, 2.0),
            Point(0.5, 0.5),
        ];
        let (p1, p2) = closest_pair(&ps);
        assert!(
            (p1 == Point(0.0, 0.0) && p2 == Point(0.5, 0.5))
                || (p1 == Point(0.5, 0.5) && p2 == Point(0.0, 0.0))
        );
    }

    #[test]
    fn test_larger_case() {
        let ps = vec![
            Point(0.0, 0.0),
            Point(10.0, 10.0),
            Point(20.0, 20.0),
            Point(5.0, 5.0),
            Point(15.0, 15.0),
            Point(5.1, 5.1),
            Point(100.0, 100.0),
        ];
        let (p1, p2) = closest_pair(&ps);
        let d = p1.distance(&p2);
        let expected_d = Point(5.0, 5.0).distance(&Point(5.1, 5.1));
        assert!((d - expected_d).abs() < 1e-9);
    }

    #[test]
    fn test_boundary_case() {
        // 境界をまたぐ最近点対のケース
        let ps = vec![
            Point(0.0, 0.0),
            Point(1.0, 0.0),
            Point(2.0, 0.0),
            Point(3.0, 0.0),
            Point(4.0, 0.0),
            Point(2.1, 0.0),
        ];
        let (p1, p2) = closest_pair(&ps);
        let d = p1.distance(&p2);
        assert!((d - 0.1).abs() < 1e-9);
    }
}
