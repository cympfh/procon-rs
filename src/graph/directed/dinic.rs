/// Graph - Directed - Dinic's MaxFlow
use crate::algebra::group::*;
use crate::algebra::hyper::*;

pub struct Dinic;
impl Dinic {
    pub fn flow<X: Copy + Eq + Ord + Group>(
        s: usize,
        t: usize,
        neigh: &Vec<Vec<(usize, X)>>,
    ) -> Hyper<X> {
        use Hyper::*;
        let n = neigh.len();
        let mut cap = vec![vec![Real(X::zero()); n]; n];
        let mut flw = vec![vec![Real(X::zero()); n]; n];
        for u in 0..n {
            for &(v, cost) in neigh[u].iter() {
                cap[u][v] = Real(cost);
            }
        }
        let mut sum = Real(X::zero());
        loop {
            let level = Dinic::levelize(&neigh, s, t, &cap, &flw);
            if level[t] == -1 {
                break;
            }
            let mut used = vec![false; n];
            sum = sum + Dinic::augment(Inf, &level, &mut used, &neigh, s, t, &cap, &mut flw);
        }
        sum
    }
    fn levelize<X: Copy + Eq + Group>(
        neigh: &Vec<Vec<(usize, X)>>,
        s: usize,
        t: usize,
        cap: &Vec<Vec<Hyper<X>>>,
        flw: &Vec<Vec<Hyper<X>>>,
    ) -> Vec<i32> {
        use Hyper::*;
        let n = neigh.len();
        let mut level = vec![-1; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            if u == t {
                break;
            }
            for &(v, _) in neigh[u].iter() {
                if level[v] == -1 && cap[u][v] - flw[u][v] != Real(X::zero()) {
                    level[v] = level[u] + 1;
                    q.push_back(v);
                }
            }
        }
        level
    }
    fn augment<X: Copy + Eq + Ord + Group>(
        limit: Hyper<X>,
        level: &Vec<i32>,
        mut used: &mut Vec<bool>,
        neigh: &Vec<Vec<(usize, X)>>,
        u: usize,
        t: usize,
        cap: &Vec<Vec<Hyper<X>>>,
        mut flw: &mut Vec<Vec<Hyper<X>>>,
    ) -> Hyper<X> {
        use Hyper::*;
        if u == t {
            limit
        } else if used[u] || limit == Real(X::zero()) {
            Real(X::zero())
        } else {
            used[u] = true;
            for &(v, _) in neigh[u].iter() {
                if level[v] > level[u] {
                    let limit2 = std::cmp::min(limit, cap[u][v] - flw[u][v]);
                    let f = Dinic::augment(limit2, &level, &mut used, &neigh, v, t, &cap, &mut flw);
                    if f > Real(X::zero()) {
                        flw[u][v] = flw[u][v] + f;
                        flw[v][u] = flw[v][u] - f;
                        used[u] = false;
                        return f;
                    }
                }
            }
            Real(X::zero())
        }
    }
}

#[cfg(test)]
mod test_dinic {
    use crate::graph::directed::dinic::*;

    #[test]
    fn test_a() {
        let neigh = vec![
            vec![(1, 1), (3, 1), (5, 1)],
            vec![(2, 1), (4, 1)],
            vec![(6, 1)],
            vec![(2, 1)],
            vec![(6, 1)],
            vec![(2, 1), (4, 1)],
            vec![],
        ];
        assert_eq!(Dinic::flow(0, 6, &neigh).unwrap(), 2);
    }

    #[test]
    fn test_b() {
        let neigh = vec![
            vec![(1, 3), (2, 3)],
            vec![(2, 2), (3, 3)],
            vec![(4, 2)],
            vec![(4, 3), (5, 2)],
            vec![(5, 3)],
            vec![],
        ];
        assert_eq!(Dinic::flow(0, 5, &neigh).unwrap(), 5);
    }
}
