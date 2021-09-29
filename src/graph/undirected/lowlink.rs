/// Graph - Undirected - LowLink for DFS-Tree
use crate::algorithm::dfs::*;
pub struct LowLink {
    pub size: usize,
    pub ord: Vec<usize>,
    pub low: Vec<usize>,
}
impl LowLink {
    pub fn new(g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let mut ord = vec![0; n];
        let mut low = vec![n; n];
        let mut dfs = DFS::default();
        dfs.push(&(0, 0));
        let mut visited = vec![false; n];
        let mut last_ord = 0;
        let mut passed = std::collections::HashSet::new();
        while let Some(s) = dfs.pop() {
            match s {
                DfsOrd::Pre((pre, u)) => {
                    if visited[u] {
                        continue;
                    }
                    passed.insert((pre, u));
                    visited[u] = true;
                    ord[u] = last_ord;
                    low[u] = last_ord;
                    last_ord += 1;
                    for &v in g[u].iter().rev() {
                        if visited[v] {
                            continue;
                        }
                        dfs.push(&(u, v));
                    }
                }
                DfsOrd::Post((_, u)) => {
                    for &v in g[u].iter().rev() {
                        if passed.contains(&(u, v)) {
                            low[u] = low[u].min(low[v]);
                        } else if !passed.contains(&(v, u)) {
                            low[u] = low[u].min(ord[v]);
                        }
                    }
                }
            }
        }
        Self { size: n, ord, low }
    }
}

#[cfg(test)]
mod test_lowlink {
    use crate::graph::undirected::lowlink::*;

    #[test]
    fn test_lowlink() {
        macro_rules! add {
            ($g:expr, $i:expr, $j:expr) => {
                $g[$i].push($j);
                $g[$j].push($i);
            };
        }

        let mut g = vec![vec![]; 10];
        add!(g, 0, 1);
        add!(g, 1, 2);
        add!(g, 1, 3);
        add!(g, 1, 4);
        add!(g, 3, 4);
        add!(g, 0, 5);
        add!(g, 5, 6);
        add!(g, 5, 9);
        add!(g, 6, 7);
        add!(g, 6, 8);
        add!(g, 6, 9);
        add!(g, 8, 9);

        let lowlink = LowLink::new(&g);
        assert_eq!(lowlink.ord, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(lowlink.low, vec![0, 1, 2, 1, 1, 5, 5, 7, 5, 5]);
    }
}
