/// Sequence - Longest (Strictly) Increasing Sequence (LIS; Young Tableu Method)
use crate::algorithm::binary_search::*;

pub fn lis<X: Copy + Ord + std::fmt::Debug>(xs: &Vec<X>) -> Vec<X> {
    if xs.len() <= 1 {
        return xs.clone();
    }
    let mut bins: Vec<Vec<X>> = vec![];
    let mut bots: Vec<X> = vec![];
    for &x in xs.iter() {
        let pos = if bots.is_empty() {
            None
        } else {
            lowerbound(0..bots.len(), &|i| bots[i] >= x)
        };
        // let pos = lowerbound(0..bots.len(), &|i| bots[i] >= x);
        if let Some(i) = pos {
            bins[i].push(x);
            bots[i] = x;
        } else {
            bins.push(vec![x]);
            bots.push(x);
        }
    }
    let m = bins.len();
    let mut last = bins[m - 1][0];
    let mut result = vec![last];
    for i in (0..m - 1).rev() {
        let j = lowerbound(0..bins[i].len(), &|j| bins[i][j] < last).unwrap();
        last = bins[i][j];
        result.push(last);
    }
    result.iter().rev().cloned().collect()
}

#[cfg(test)]
mod test_lis {
    use crate::sequence::lis::*;
    #[test]
    fn test() {
        assert_eq!(lis(&vec![1]), vec![1]);
        assert_eq!(lis(&vec![1, 1, 1]), vec![1]);
        assert_eq!(lis(&vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(lis(&vec![4, 2, 3, 1, 5]), vec![2, 3, 5]);
        assert_eq!(lis(&vec![3, 1, 2, 2, 2]), vec![1, 2]);
        assert_eq!(lis(&vec![3, 1, 3]), vec![1, 3]);
        assert_eq!(lis(&vec![1, 2, 1, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_empty() {
        let xs: Vec<usize> = vec![];
        assert_eq!(lis(&xs), xs);
    }
}
