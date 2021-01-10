/// Graph - Directed - Dinic's MaxFlow
use crate::algebra::group::*;
use crate::algebra::hyper::*;

pub struct Dinic<X> {
    size: usize,
    s: usize,
    t: usize,
    g: Vec<Vec<(usize, Hyper<X>)>>,
}
impl<X: std::fmt::Debug + Copy + Eq + Ord + Group> Dinic<X> {
    pub fn new(s: usize, t: usize, neigh: &Vec<Vec<(usize, Hyper<X>)>>) -> Self {
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
            let level = self.levelize(&flw);
            if level[self.t].is_none() {
                break;
            }
            sum = sum + self.augment(Hyper::Inf, &mut flw, &level, self.s);
        }
        sum
    }
    fn levelize(&self, flw: &Vec<Vec<Hyper<X>>>) -> Vec<Option<usize>> {
        let mut level = vec![None; self.size];
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.s);
        level[self.s] = Some(0);
        while let Some(u) = q.pop_front() {
            if level[u].is_none() || u == self.t {
                break;
            }
            for &(v, cap) in self.g[u].iter() {
                if level[v].is_none() && cap > flw[u][v] {
                    level[v] = level[u].map(|x| x + 1);
                    q.push_back(v);
                }
            }
        }
        level
    }
    fn augment(
        &self,
        limit: Hyper<X>,
        mut flw: &mut Vec<Vec<Hyper<X>>>,
        level: &Vec<Option<usize>>,
        u: usize,
    ) -> Hyper<X> {
        if u == self.t {
            limit
        } else {
            for &(v, cap) in self.g[u].iter() {
                if level[v] > level[u] {
                    let limit = std::cmp::min(limit, cap - flw[u][v]);
                    let f = self.augment(limit, &mut flw, &level, v);
                    if f > Hyper::zero() {
                        flw[u][v] = flw[u][v] + f;
                        flw[v][u] = flw[v][u] - f;
                        return f;
                    }
                }
            }
            Hyper::zero()
        }
    }
}

#[cfg(test)]
mod test_dinic {
    use crate::algebra::hyper::Hyper::*;
    use crate::graph::directed::dinic::*;

    #[test]
    fn test_a() {
        let neigh = vec![
            vec![(1, Real(1)), (3, Real(1)), (5, Real(1))],
            vec![(2, Real(1)), (4, Real(1))],
            vec![(6, Real(1))],
            vec![(2, Real(1))],
            vec![(6, Real(1))],
            vec![(2, Real(1)), (4, Real(1))],
            vec![],
        ];
        assert_eq!(Dinic::new(0, 6, &neigh).maxflow(), Real(2));
    }

    #[test]
    fn test_b() {
        let neigh = vec![
            vec![(1, Real(3)), (2, Real(3))],
            vec![(2, Real(2)), (3, Real(3))],
            vec![(4, Real(2))],
            vec![(4, Real(3)), (5, Real(2))],
            vec![(5, Real(3))],
            vec![],
        ];
        assert_eq!(Dinic::new(0, 5, &neigh).maxflow(), Real(5));
    }
}
