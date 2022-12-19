/// Algorithm - Gauss-Jorndan elimination
use crate::num::float::*;

/// Solve: a x = b
/// where
///   a is NxN matrix
///   b is N vector
pub fn gauss_jordan(a: Vec<Vec<Float>>, b: Vec<Float>) -> Option<Vec<Float>> {
    let n = a.len();
    assert!(b.len() == n);
    let mut a = a.clone();
    for i in 0..n {
        a[i].push(b[i]);
    }
    for i in 0..n {
        let pivot = (i..n).max_by_key(|&k| a[k][i].abs()).unwrap();
        a.swap(i, pivot);
        if a[i][i].abs() <= Float(1e-6) {
            return None;
        }
        {
            let base = a[i][i];
            for j in i..=n {
                a[i][j] /= base;
            }
        }
        // 消去
        for ii in 0..n {
            if ii == i {
                continue;
            }
            for j in i + 1..=n {
                let elim = a[ii][i] * a[i][j];
                a[ii][j] -= elim;
            }
        }
    }
    Some((0..n).map(|i| a[i][n]).collect::<Vec<_>>())
}

#[cfg(test)]
mod test_gauss_jordan {
    use crate::algorithm::gauss_jordan::*;
    #[test]
    fn test_1() {
        // x + y = 0
        // x + 2y = 10
        let a = vec![vec![Float(1.0), Float(1.0)], vec![Float(1.0), Float(2.0)]];
        let b = vec![Float(0.0), Float(10.0)];
        assert_eq!(gauss_jordan(a, b), Some(vec![Float(-10.0), Float(10.0)]));
    }
    #[test]
    fn test_2() {
        // 2 x + y = -1
        // -x + 3 y = 11
        let a = vec![vec![Float(2.0), Float(1.0)], vec![Float(-1.0), Float(3.0)]];
        let b = vec![Float(-1.0), Float(11.0)];
        assert_eq!(gauss_jordan(a, b), Some(vec![Float(-2.0), Float(3.0)]));
    }
    #[test]
    fn test_3() {
        // x + y = 0
        // x + y = 10
        let a = vec![vec![Float(1.0), Float(1.0)], vec![Float(1.0), Float(1.0)]];
        let b = vec![Float(0.0), Float(10.0)];
        assert_eq!(gauss_jordan(a, b), None);
    }
    #[test]
    fn test_4() {
        // x + y = 1
        // x + y = 1
        let a = vec![vec![Float(1.0), Float(1.0)], vec![Float(1.0), Float(1.0)]];
        let b = vec![Float(1.0), Float(1.0)];
        assert_eq!(gauss_jordan(a, b), None);
    }
}
