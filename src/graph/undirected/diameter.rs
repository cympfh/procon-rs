/// Graph - Undirected - Diameter
fn diameter(neigh: &Vec<Vec<usize>>) -> usize {
    let n = neigh.len();
    let mut d = vec![vec![n * 2 + 1; n]; n];
    for i in 0..n {
        d[i][i] = 0
    }
    for i in 0..n {
        for &j in neigh[i].iter() {
            d[i][j] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let w = d[i][k] + d[k][j];
                if w < d[i][j] {
                    d[i][j] = w;
                }
            }
        }
    }
    (1..n)
        .map(|i| (0..i).map(|j| d[i][j]).max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod test_diameter {
    use crate::graph::undirected::diameter::*;
    #[test]
    fn cycle4() {
        assert_eq!(
            diameter(&vec![vec![3, 1], vec![0, 2], vec![1, 3], vec![2, 0],]),
            2
        );
    }
    #[test]
    fn cycle5() {
        assert_eq!(
            diameter(&vec![
                vec![4, 1],
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 0],
            ]),
            2
        );
    }
    #[test]
    fn uni() {
        assert_eq!(
            diameter(&vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0],]),
            2
        );
    }
}
