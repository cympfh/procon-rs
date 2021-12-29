/// Graph - Warshall-Floyd
use crate::algebra::group_additive::*;
use crate::algebra::hyper::*;

pub fn warshall_floyd<X: Copy + AGroup + PartialOrd>(d: &mut [Vec<Hyper<X>>]) {
    let n = d.len();
    for i in 0..n {
        d[i][i] = Hyper::<X>::zero();
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let w = d[i][k] + d[k][j];
                if w < d[i][j] {
                    d[i][j] = w;
                }
            }
        }
    }
}

#[cfg(test)]
mod test_warshall_floyd {
    use crate::algebra::hyper::Hyper::*;
    use crate::graph::shortest::warshall_floyd::*;

    #[test]
    fn test_circle_undirected() {
        let mut neigh: Vec<Vec<Hyper<i64>>> = vec![
            vec![Inf, Real(1), Inf, Inf],
            vec![Inf, Inf, Real(1), Inf],
            vec![Inf, Inf, Inf, Real(1)],
            vec![Real(1), Inf, Inf, Inf],
        ];
        let expected = [
            [Real(0), Real(1), Real(2), Real(3)],
            [Real(3), Real(0), Real(1), Real(2)],
            [Real(2), Real(3), Real(0), Real(1)],
            [Real(1), Real(2), Real(3), Real(0)],
        ];
        warshall_floyd(&mut neigh);
        assert_eq!(neigh, expected);
    }

    #[test]
    fn test_circle_directed() {
        let mut neigh: Vec<Vec<Hyper<i64>>> = vec![
            vec![Inf, Real(1), Inf, Real(1)],
            vec![Real(1), Inf, Real(1), Inf],
            vec![Inf, Real(1), Inf, Real(1)],
            vec![Real(1), Inf, Real(1), Inf],
        ];
        let expected = [
            [Real(0), Real(1), Real(2), Real(1)],
            [Real(1), Real(0), Real(1), Real(2)],
            [Real(2), Real(1), Real(0), Real(1)],
            [Real(1), Real(2), Real(1), Real(0)],
        ];
        warshall_floyd(&mut neigh);
        assert_eq!(neigh, expected);
    }

    #[test]
    fn test_circle_unconnected() {
        let mut neigh: Vec<Vec<Hyper<i64>>> = vec![
            vec![Inf, Real(1), Inf, Inf],
            vec![Real(1), Inf, Inf, Inf],
            vec![Inf, Inf, Inf, Real(1)],
            vec![Inf, Inf, Real(1), Inf],
        ];
        let expected = [
            [Real(0), Real(1), Inf, Inf],
            [Real(1), Real(0), Inf, Inf],
            [Inf, Inf, Real(0), Real(1)],
            [Inf, Inf, Real(1), Real(0)],
        ];
        warshall_floyd(&mut neigh);
        assert_eq!(neigh, expected);
    }
}
