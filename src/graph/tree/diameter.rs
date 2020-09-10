/// Graph - Tree - Diameter
fn diameter(tree: &Vec<Vec<usize>>) -> usize {
    use std::cmp::Reverse;
    let n = tree.len();
    let mut s = 0;
    let mut maxd = 0;
    for _ in 0..2 {
        let mut memo = vec![n * 10; n];
        memo[s] = 0;
        let mut q = std::collections::BinaryHeap::new();
        q.push((Reverse(0), s));
        while let Some((_, u)) = q.pop() {
            for &v in tree[u].iter() {
                if memo[v] > memo[u] + 1 {
                    memo[v] = memo[u] + 1;
                    q.push((Reverse(memo[v]), v));
                }
            }
        }
        s = (0..n).map(|i| (memo[i], i)).max().unwrap().1;
        maxd = *memo.iter().max().unwrap();
    }
    maxd
}

#[cfg(test)]
mod test_diameter {
    use crate::graph::tree::diameter::*;

    #[test]
    fn it_works() {
        assert_eq!(
            diameter(&vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]]),
            3
        );
    }
}
