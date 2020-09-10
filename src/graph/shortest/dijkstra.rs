/// Graph - Dijkstra
use crate::algebra::group::*;
use crate::algebra::hyper::*;

fn dijkstra<Cost: Copy + Group + Ord>(
    s: usize,
    neigh: &Vec<Vec<(usize, Cost)>>,
) -> Vec<Hyper<Cost>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = neigh.len();
    let mut d: Vec<Hyper<Cost>> = vec![Hyper::Inf; n];
    let mut q = BinaryHeap::new();
    d[s] = Hyper::Real(Cost::zero());
    q.push((Reverse(d[s]), s));
    while let Some((_, u)) = q.pop() {
        for &(v, cost) in neigh[u].iter() {
            if d[v] > d[u] + cost {
                d[v] = d[u] + cost;
                q.push((Reverse(d[v]), v));
            }
        }
    }
    d
}

#[cfg(test)]
mod test_dijkstra {
    use crate::algebra::hyper::Hyper::*;
    use crate::graph::shortest::dijkstra::*;

    #[test]
    fn test_circle_undirected() {
        type Cost = i32;
        let neigh: Vec<Vec<(usize, Cost)>> = vec![
            vec![(1, 1), (4, 1)],
            vec![(2, 1), (0, 1)],
            vec![(3, 1), (1, 1)],
            vec![(4, 1), (2, 1)],
            vec![(0, 1), (3, 1)],
        ];
        let expected = vec![Real(0), Real(1), Real(2), Real(2), Real(1)];
        assert_eq!(dijkstra(0, &neigh), expected);
    }

    #[test]
    fn test_circle_directed() {
        type Cost = i32;
        let neigh: Vec<Vec<(usize, Cost)>> = vec![
            vec![(1, 1)],
            vec![(2, 1)],
            vec![(3, 1)],
            vec![(4, 1)],
            vec![(0, 1)],
        ];
        let expected = vec![Real(0), Real(1), Real(2), Real(3), Real(4)];
        assert_eq!(dijkstra(0, &neigh), expected);
    }

    #[test]
    fn test_unconnected() {
        type Cost = i32;
        let neigh: Vec<Vec<(usize, Cost)>> = vec![
            vec![(1, 1)],
            vec![(0, 1)],
            vec![(3, 1)],
            vec![(4, 1)],
            vec![(2, 1)],
        ];
        let expected = vec![Real(0), Real(1), Inf, Inf, Inf];
        assert_eq!(dijkstra(0, &neigh), expected);
    }
}
