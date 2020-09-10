/// Graph - Undirected - is Bipartite Graph?
use crate::set::union_find::*;

fn is_bipartite_graph(neigh: &[Vec<usize>]) -> bool {
    let n = neigh.len();
    let mut uf = UnionFind::new(n * 2);
    for u in 0..n {
        for &v in neigh[u].iter() {
            uf.merge(u * 2, v * 2 + 1);
            uf.merge(u * 2 + 1, v * 2);
        }
    }
    for u in 0..n {
        if uf.is_same(u * 2, u * 2 + 1) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test_is_bipartite_graph {
    use crate::graph::undirected::is_bipartite_graph::*;

    #[test]
    fn it_works() {
        assert!(is_bipartite_graph(&vec![vec![]]));
        assert!(is_bipartite_graph(&[vec![1], vec![0]]));
        assert!(is_bipartite_graph(&[vec![1], vec![2], vec![]]));
        assert!(is_bipartite_graph(&[vec![1], vec![2], vec![3], vec![]]));
        assert!(is_bipartite_graph(&[vec![1], vec![2], vec![3], vec![0]]));

        assert!(!is_bipartite_graph(&[vec![1], vec![2], vec![0]]));
        assert!(!is_bipartite_graph(&[
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![0]
        ]));
    }
}
