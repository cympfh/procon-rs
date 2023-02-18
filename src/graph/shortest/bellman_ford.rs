/// Graph - Bellman-Ford
use crate::algebra::group_additive::*;
use crate::algebra::hyper::*;

pub fn bellman_ford<X: Copy + AGroup + PartialOrd>(
    s: usize,
    t: usize,
    neigh: &[Vec<(usize, X)>],
) -> Hyper<X> {
    use Hyper::*;
    let n = neigh.len();
    let mut dist = vec![Inf; n];
    dist[s] = Real(X::zero());
    // Yen's
    let edges: Vec<(usize, usize, Hyper<X>)> = {
        let mut edges_f = vec![];
        let mut edges_b = vec![];
        for u in 0..n {
            for &(v, cost) in neigh[u].iter() {
                if u <= v {
                    edges_f.push((u, v, Real(cost)));
                } else {
                    edges_b.push((u, v, Real(cost)));
                }
            }
        }
        edges_f.sort_by_key(|&item| item.0);
        edges_b.sort_by_key(|&item| item.0);
        edges_b.reverse();
        edges_f.iter().chain(edges_b.iter()).map(|&p| p).collect()
    };
    for _ in 1..n {
        for &(u, v, cost) in edges.iter() {
            if dist[v] > dist[u] + cost {
                dist[v] = dist[u] + cost;
            }
        }
    }
    for u in 0..n {
        for &(v, cost) in neigh[u].iter() {
            if dist[v] > dist[u] + cost {
                dist[u] = NegInf;
            }
        }
    }
    for _ in 1..n {
        for &(u, v, cost) in edges.iter() {
            if dist[v] > dist[u] + cost {
                dist[v] = dist[u] + cost;
            }
        }
    }
    dist[t]
}

#[cfg(test)]
mod test_bellman_ford {
    use crate::algebra::hyper::Hyper::*;
    use crate::graph::shortest::bellman_ford::*;

    #[test]
    fn test_cycle_contains_negative_edge() {
        let neigh: Vec<Vec<(usize, i64)>> = vec![
            vec![(1, 1), (3, -1)],
            vec![(0, 1), (2, 1)],
            vec![(1, 1), (3, 1)],
            vec![(2, 1)],
        ];
        assert_eq!(bellman_ford(0, 1, &neigh), Real(1));
        assert_eq!(bellman_ford(0, 2, &neigh), Real(0));
        assert_eq!(bellman_ford(0, 3, &neigh), Real(-1));
        assert_eq!(bellman_ford(3, 0, &neigh), Real(3));
    }

    #[test]
    fn test_contains_negative_cycle() {
        let neigh: Vec<Vec<(usize, i128)>> =
            vec![vec![(1, -1)], vec![(0, -1), (2, 1)], vec![(1, 1)]];
        assert_eq!(bellman_ford(0, 1, &neigh), NegInf);
        assert_eq!(bellman_ford(0, 2, &neigh), NegInf);
    }
}
