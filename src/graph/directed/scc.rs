/// Graph - Directed - Strongly Connected Component (SCC)
/// convert a DiGraph to a DAG
/// scc: (g) -> (cmp, dag)
///   where
///     `g` is a neighbor list of DiGraph
///     `cmp` is mapping vector; cmp[DiGraph-Vertex-Index] = DAG-Vertex-Index
///     `dag` is a neighbor list of DAG
pub fn scc(g: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = g.len();

    // Post-order traversal
    let mut po = vec![];
    {
        fn dfs(u: usize, g: &Vec<Vec<usize>>, mut used: &mut Vec<bool>, mut po: &mut Vec<usize>) {
            if used[u] {
                return;
            }
            used[u] = true;
            for &v in g[u].iter() {
                if !used[v] {
                    dfs(v, &g, &mut used, &mut po);
                }
            }
            po.push(u);
        }
        let mut used = vec![false; n];
        for u in 0..n {
            dfs(u, &g, &mut used, &mut po);
        }
    }

    let mut g_r = vec![vec![]; n];
    for u in 0..n {
        for &v in g[u].iter() {
            g_r[v].push(u);
        }
    }

    // Components
    let mut cmp = vec![0; n];
    let m;
    {
        let mut used = vec![false; n];
        let mut k = 0;
        po.reverse();
        for &u in po.iter() {
            let mut stack = vec![u];
            if used[u] {
                continue;
            }
            while let Some(v) = stack.pop() {
                if used[v] {
                    continue;
                }
                used[v] = true;
                cmp[v] = k;
                for &w in g_r[v].iter() {
                    stack.push(w)
                }
            }
            k += 1;
        }
        m = k;
    }

    // DAG
    let mut dag = vec![vec![]; m];
    for u in 0..n {
        let u2 = cmp[u];
        for &v in g[u].iter() {
            let v2 = cmp[v];
            if u2 != v2 {
                dag[u2].push(v2)
            }
        }
    }
    for u in 0..m {
        dag[u].sort();
        dag[u].dedup();
    }

    (cmp, dag)
}

#[cfg(test)]
mod test_scc {
    use crate::graph::directed::scc::*;

    #[test]
    fn it_works() {
        let g = vec![vec![1], vec![0, 2], vec![3], vec![2]];
        let (cmp, dag) = scc(&g);
        assert_eq!(cmp, vec![0, 0, 1, 1]);
        assert_eq!(dag, vec![vec![1], vec![]]);
    }

    #[test]
    fn it_connected() {
        let g = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let (cmp, _dag) = scc(&g);
        assert_eq!(cmp, vec![0, 0, 0, 0]);
    }
}
