/// Graph - Tree - Lowest Common Ancestor (LCA)
pub struct LCA {
    depth: Vec<usize>,
    parent: Vec<Vec<Option<usize>>>,
}
impl LCA {
    /// Directed(Parent -> Child)
    pub fn new(tree: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = tree.len();
        let log2n = (0..n).map(|i| n >> i).take_while(|&x| x > 0).count();
        let mut parent = vec![vec![None; log2n]; n];
        let depth: Vec<usize> = {
            let mut depth = vec![None; n];
            let mut stack = vec![(0, root)];
            while let Some((d, u)) = stack.pop() {
                if depth[u].is_some() {
                    continue;
                }
                depth[u] = Some(d);
                for &v in tree[u].iter() {
                    parent[v][0] = Some(u);
                    if depth[v].is_some() {
                        continue;
                    }
                    stack.push((d + 1, v));
                }
            }
            depth.iter().map(|d| d.unwrap()).collect()
        };
        for k in 1..log2n {
            for u in 0..n {
                if let Some(v) = parent[u][k - 1] {
                    parent[u][k] = parent[v][k - 1];
                }
            }
        }
        LCA {
            depth: depth,
            parent: parent,
        }
    }
    pub fn get(&self, u: usize, v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            return self.get(v, u);
        }
        let mut u = u;
        let mut v = v;
        while self.depth[u] < self.depth[v] {
            v = self.parent[v][0].unwrap();
        }
        if u == v {
            return u;
        }
        let log2n = self.parent[0].len();
        for k in (0..log2n).rev() {
            match (self.parent[u][k], self.parent[v][k]) {
                (Some(s), Some(t)) => {
                    if s != t {
                        u = s;
                        v = t;
                    }
                }
                _ => {}
            }
        }
        self.parent[u][0].unwrap()
    }
}

#[cfg(test)]
mod test_lca {
    use crate::graph::tree::lca::*;

    #[test]
    fn it_works() {
        let lca = LCA::new(&vec![vec![1, 2], vec![3, 4], vec![], vec![], vec![]], 0);
        assert_eq!(lca.get(0, 2), 0);
        assert_eq!(lca.get(1, 2), 0);
        assert_eq!(lca.get(1, 3), 1);
        assert_eq!(lca.get(3, 4), 1);
        assert_eq!(lca.get(4, 4), 4);
    }
}
