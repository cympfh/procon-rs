/// Graph - Directed - Strongly Connected Component (SCC)
/// convert a DiGraph to a DAG
/// scc: (neigh) -> (cmp, dag)
///   where
///     neigh is a neigh list of DiGraph
///     cmp is mapping vector; cmp[DiGraph-Vertex-Index] = DAG-Vertex-Index
///     dag is a neigh list of DAG
fn scc(neigh: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = neigh.len();

    // Post-order traversal
    let mut po = vec![];
    {
        let mut used = vec![false; n];
        for u in 0..n {
            let mut stack = vec![u];
            let mut pre = vec![];
            while let Some(v) = stack.pop() {
                if used[v] {
                    continue;
                }
                used[v] = true;
                pre.push(v);
                for &w in neigh[v].iter() {
                    stack.push(w)
                }
            }
            pre.reverse();
            po.extend(pre);
        }
    }

    let mut neigh_r = vec![vec![]; n];
    for u in 0..n {
        for &v in neigh[u].iter() {
            neigh_r[v].push(u);
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
                for &w in neigh_r[v].iter() {
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
        for &v in neigh[u].iter() {
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
        let neigh = vec![vec![1], vec![0, 2], vec![3], vec![2]];
        let (cmp, dag) = scc(&neigh);
        assert_eq!(cmp, vec![0, 0, 1, 1]);
        assert_eq!(dag, vec![vec![1], vec![]]);
    }

    #[test]
    fn it_connected() {
        let neigh = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let (cmp, dag) = scc(&neigh);
        assert_eq!(cmp, vec![0, 0, 0, 0]);
    }
}
