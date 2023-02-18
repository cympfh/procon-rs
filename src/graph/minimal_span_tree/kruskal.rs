/// Graph - Minimal Spanning Tree - Kruskal Algorithm
use crate::algebra::group_additive::*;
use crate::set::union_find::*;

pub fn kruskal<Cost: Copy + AGroup + Ord>(neigh: &Vec<Vec<(usize, Cost)>>) -> Cost {
    let n = neigh.len();
    let mut total = Cost::zero();
    let mut uf = UnionFind::new(n);
    let mut q = vec![];
    for u in 0..n {
        for &(v, cost) in neigh[u].iter() {
            q.push((cost, u, v));
        }
    }
    q.sort();
    for &(cost, i, j) in q.iter() {
        if uf.is_same(i, j) {
            continue;
        }
        uf.merge(i, j);
        total = total + cost;
    }
    total
}

#[cfg(test)]
mod test_kruskal {
    use crate::graph::minimal_span_tree::kruskal::*;

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
        let neigh: Vec<Vec<(usize, i64)>> = vec![
            vec![(1, 7), (3, 5)],
            vec![(2, 8), (3, 9), (4, 7)],
            vec![(4, 5)],
            vec![(4, 15), (5, 6)],
            vec![(5, 8), (6, 9)],
            vec![(6, 11)],
            vec![],
        ];
        assert_eq!(kruskal(&undirectize(&neigh)), 39);
    }
}
