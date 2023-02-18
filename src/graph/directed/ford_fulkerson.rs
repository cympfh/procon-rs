/// Graph - Directed - Ford-Fullkerson's MaxFlow - O(FE)
use crate::algebra::group_additive::*;
use crate::algebra::hyper::*;

pub struct FordFulkerson<X> {
    size: usize,
    s: usize,
    t: usize,
    g: Vec<Vec<(usize, Hyper<X>)>>,
}

impl<X: std::fmt::Debug + Copy + Eq + Ord + AGroup> FordFulkerson<X> {
    pub fn new(s: usize, t: usize, neigh: &[Vec<(usize, Hyper<X>)>]) -> Self {
        let size = neigh.len();
        let mut g = vec![vec![]; size];
        for u in 0..size {
            for &(v, cap) in neigh[u].iter() {
                g[u].push((v, cap));
                g[v].push((u, Hyper::zero()));
            }
        }
        Self { size, s, t, g }
    }
    pub fn maxflow(&self) -> Hyper<X> {
        let mut sum = Hyper::zero();
        let mut flw = vec![vec![Hyper::zero(); self.size]; self.size];
        loop {
            let mut visited = vec![false; self.size];
            let df = self.augment(Hyper::Inf, &mut flw, &mut visited, self.s);
            if df <= Hyper::zero() {
                break;
            }
            sum += df;
        }
        sum
    }
    fn augment(
        &self,
        limit: Hyper<X>,
        mut flw: &mut [Vec<Hyper<X>>],
        mut visited: &mut [bool],
        u: usize,
    ) -> Hyper<X> {
        if u == self.t {
            return limit;
        }
        visited[u] = true;
        for &(v, cap) in self.g[u].iter() {
            if visited[v] {
                continue;
            }
            if cap > flw[u][v] {
                let limit = std::cmp::min(limit, cap - flw[u][v]);
                let f = self.augment(limit, &mut flw, &mut visited, v);
                if f > Hyper::zero() {
                    flw[u][v] += f;
                    flw[v][u] -= f;
                    return f;
                }
            }
        }
        Hyper::zero()
    }
}

#[cfg(test)]
mod test_ffa {
    use crate::algebra::hyper::Hyper::*;
    use crate::graph::directed::ford_fulkerson::*;

    #[test]
    fn test_a() {
        let neigh = vec![
            vec![(1, Real(1_i64)), (3, Real(1)), (5, Real(1))],
            vec![(2, Real(1)), (4, Real(1))],
            vec![(6, Real(1))],
            vec![(2, Real(1))],
            vec![(6, Real(1))],
            vec![(2, Real(1)), (4, Real(1))],
            vec![],
        ];
        assert_eq!(FordFulkerson::new(0, 6, &neigh).maxflow(), Real(2));
    }

    #[test]
    fn test_b() {
        let neigh = vec![
            vec![(1, Real(3_i128)), (2, Real(3))],
            vec![(2, Real(2)), (3, Real(3))],
            vec![(4, Real(2))],
            vec![(4, Real(3)), (5, Real(2))],
            vec![(5, Real(3))],
            vec![],
        ];
        assert_eq!(FordFulkerson::new(0, 5, &neigh).maxflow(), Real(5));
    }
}
