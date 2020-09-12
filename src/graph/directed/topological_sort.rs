/// Graph - Directed - Topological Sort

pub struct Topological;

impl Topological {
    pub fn sort(neigh: &Vec<Vec<usize>>) -> Vec<usize> {
        let n = neigh.len();
        let mut rd = vec![vec![]; n];
        for u in 0..n {
            for &v in neigh[u].iter() {
                rd[v].push(u);
            }
        }
        let mut used = vec![false; n];
        let mut ord = vec![];
        for u in 0..n {
            Self::visit(u, &mut used, &rd, &mut ord);
        }
        ord
    }
    fn visit(u: usize, mut used: &mut Vec<bool>, rd: &Vec<Vec<usize>>, mut ord: &mut Vec<usize>) {
        if used[u] {
            return;
        }
        used[u] = true;
        for &v in rd[u].iter() {
            Self::visit(v, &mut used, &rd, &mut ord);
        }
        ord.push(u);
    }
}

#[cfg(test)]
mod test_topological_sort {
    use crate::graph::directed::topological_sort::*;

    fn autocheck(neigh: Vec<Vec<usize>>) {
        let ord = Topological::sort(&neigh);
        let n = neigh.len();
        for u in 0..n {
            let i = ord.iter().position(|&x| x == u).unwrap();
            for &v in neigh[u].iter() {
                let j = ord.iter().position(|&x| x == v).unwrap();
                assert!(i < j);
            }
        }
    }

    #[test]
    fn it_works() {
        let graphs = vec![
            vec![vec![1], vec![], vec![1]],
            vec![vec![], vec![0], vec![1]],
            vec![vec![2], vec![2], vec![3, 4], vec![], vec![]],
            vec![vec![2], vec![2], vec![], vec![2], vec![2]],
            vec![vec![2], vec![2], vec![], vec![2], vec![2]],
            vec![vec![], vec![], vec![0, 1, 3, 4], vec![], vec![]],
        ];
        for neigh in graphs {
            autocheck(neigh);
        }
    }
}
