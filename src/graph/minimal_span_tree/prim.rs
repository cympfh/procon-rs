/// Graph - Minimal Span Tree - Prim
use crate::algebra::group::*;

fn prim<Cost: Copy + Group + Ord>(neigh: &Vec<Vec<(usize, Cost)>>) -> Cost {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = neigh.len();
    let mut total = Cost::zero();
    let mut used = vec![false; n];
    let mut q = BinaryHeap::new();
    used[0] = true;
    for &(v, cost) in neigh[0].iter() {
        q.push((Reverse(cost), 0, v));
    }
    while let Some((Reverse(cost), u, v)) = q.pop() {
        if used[u] && used[v] {
            continue;
        }
        total = total + cost;
        if !used[u] {
            used[u] = true;
            for &(w, cost) in neigh[u].iter() {
                q.push((Reverse(cost), u, w));
            }
        } else if !used[v] {
            used[v] = true;
            for &(w, cost) in neigh[v].iter() {
                q.push((Reverse(cost), v, w));
            }
        }
    }
    total
}

#[cfg(test)]
mod test_prim {
    use crate::graph::minimal_span_tree::prim::*;

    fn undirectize<Cost: Copy>(g: &Vec<Vec<(usize, Cost)>>) -> Vec<Vec<(usize, Cost)>> {
        let n = g.len();
        let mut h = vec![vec![]; n];
        for u in 0..n {
            for &(v, cost) in g[u].iter() {
                h[u].push((v, cost));
                h[v].push((u, cost));
            }
        }
        h
    }

    #[test]
    fn it_works() {
        assert_eq!(
            prim(&undirectize(&vec![
                vec![(1, 7), (3, 5)],
                vec![(2, 8), (3, 9), (4, 7)],
                vec![(4, 5)],
                vec![(4, 15), (5, 6)],
                vec![(5, 8), (6, 9)],
                vec![(6, 11)],
                vec![],
            ])),
            39
        );
    }
}
