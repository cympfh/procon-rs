/// Graph class
use crate::algebra::hyper::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub n: usize,
    pub data: Vec<Vec<usize>>,
    pub cost: Vec<Vec<Hyper<i64>>>,
}
impl Graph {
    pub fn new(n: usize) -> Self {
        let data = vec![vec![]; n];
        let cost = vec![vec![]; n];
        Self { n, data, cost }
    }
    /// undirected edge
    pub fn uedge(&mut self, u: usize, v: usize) {
        self.dedge(u, v);
        self.dedge(v, u);
    }
    /// directed edge
    pub fn dedge(&mut self, u: usize, v: usize) {
        self.dedge_with_cost(u, v, Hyper::Real(1));
    }
    /// undirected edge + cost
    pub fn uedge_with_cost(&mut self, u: usize, v: usize, cost: Hyper<i64>) {
        self.dedge_with_cost(u, v, cost);
        self.dedge_with_cost(v, u, cost);
    }
    /// directed edge + cost
    pub fn dedge_with_cost(&mut self, u: usize, v: usize, cost: Hyper<i64>) {
        self.data[u].push(v);
        self.cost[u].push(cost);
    }
    /// adj list
    pub fn neigh(&self, u: usize) -> Vec<usize> {
        self.data[u].to_vec()
    }
    /// adj list + cost
    pub fn neigh_with_cost(&self, u: usize) -> Vec<(usize, Hyper<i64>)> {
        self.data[u]
            .iter()
            .cloned()
            .zip(self.cost[u].iter().cloned())
            .collect()
    }
    /// edges have been costed?
    pub fn is_costed(&self) -> bool {
        self.cost.iter().any(|v| !v.is_empty())
    }
    pub fn reverse(&self) -> Self {
        -(self.clone())
    }
    /// undirected -> directed rooted tree
    pub fn to_rooted(&self, root: usize) -> Self {
        let mut r = Graph::new(self.n);
        let mut stack = vec![root];
        let mut visited = vec![false; self.n];
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for (v, cost) in self.neigh_with_cost(u) {
                if visited[v] {
                    continue;
                }
                r.dedge_with_cost(u, v, cost);
                stack.push(v);
            }
        }
        r
    }
    /// -> adj matrix
    pub fn to_matrix(&self) -> Vec<Vec<Hyper<i64>>> {
        let mut mat = vec![vec![Hyper::Inf; self.n]; self.n];
        for u in 0..self.n {
            for (v, cost) in self.neigh_with_cost(u) {
                mat[u][v] = mat[u][v].min(cost);
            }
            mat[u][u] = Hyper::Real(0);
        }
        mat
    }
}
impl std::ops::Neg for Graph {
    type Output = Graph;
    fn neg(self) -> Self::Output {
        let mut r = Graph::new(self.n);
        for u in 0..self.n {
            for (v, cost) in self.neigh_with_cost(u) {
                r.dedge_with_cost(v, u, cost);
            }
        }
        r
    }
}

#[cfg(test)]
mod test_graph {
    use crate::algebra::hyper::Hyper;
    use crate::graph::graph::Graph;
    use Hyper::*;

    #[test]
    fn test_undirected_uncosted() {
        let mut g = Graph::new(3);
        g.uedge(0, 1);
        g.uedge(0, 2);

        assert_eq!(g.neigh(0), vec![1, 2]);
        assert_eq!(g.neigh(1), vec![0]);
        assert_eq!(g.neigh(2), vec![0]);

        // costs are considered as 1.
        assert_eq!(g.neigh_with_cost(0), vec![(1, Real(1)), (2, Real(1))]);
        assert_eq!(g.neigh_with_cost(1), vec![(0, Real(1))]);
        assert_eq!(g.neigh_with_cost(2), vec![(0, Real(1))]);

        let tree = g.to_rooted(0);
        assert_eq!(tree.neigh(0), vec![1, 2]);
        assert_eq!(tree.neigh(1), vec![]);
        assert_eq!(tree.neigh(2), vec![]);

        let tree = g.to_rooted(2);
        assert_eq!(tree.neigh(0), vec![1]);
        assert_eq!(tree.neigh(1), vec![]);
        assert_eq!(tree.neigh(2), vec![0]);
    }

    #[test]
    fn test_directed_costed() {
        let mut g = Graph::new(3);
        g.dedge_with_cost(0, 1, Real(2));
        g.dedge_with_cost(1, 2, Real(3));

        let g_rev = g.reverse();
        assert_eq!(g_rev.neigh_with_cost(0), vec![]);
        assert_eq!(g_rev.neigh_with_cost(1), vec![(0, Real(2))]);
        assert_eq!(g_rev.neigh_with_cost(2), vec![(1, Real(3))]);
    }

    #[test]
    fn test_matrix() {
        let mut g = Graph::new(3);
        g.uedge(0, 1);
        g.uedge(0, 2);
        let mat = g.to_matrix();
        assert_eq!(
            mat,
            vec![
                vec![Real(0), Real(1), Real(1)],
                vec![Real(1), Real(0), Inf],
                vec![Real(1), Inf, Real(0)],
            ]
        );

        let mut g = Graph::new(3);
        g.dedge_with_cost(0, 2, Real(5));
        g.dedge_with_cost(1, 2, Real(7));
        let mat = g.to_matrix();
        assert_eq!(
            mat,
            vec![
                vec![Real(0), Inf, Real(5)],
                vec![Inf, Real(0), Real(7)],
                vec![Inf, Inf, Real(0)],
            ]
        );
    }
}
