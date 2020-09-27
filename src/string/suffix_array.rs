/// String - Suffix Array (O(n log n log n))
pub fn suffix_array<T: Eq + Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n <= 1 {
        return (0..n).collect();
    }
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = (0..n).collect();
    {
        let mut ts: Vec<(usize, &T)> = s.iter().enumerate().collect();
        ts.sort_by_key(|&item| item.1);
        rank[ts[0].0] = 0;
        for i in 1..n {
            rank[ts[i].0] = rank[ts[i - 1].0] + if ts[i - 1].1 == ts[i].1 { 0 } else { 1 };
        }
    }
    use std::cmp::Ordering;
    fn compare(i: usize, j: usize, k: usize, rank: &Vec<usize>) -> Ordering {
        match (rank[i], rank[j], rank.get(i + k), rank.get(j + k)) {
            (x, y, _, _) if x < y => Ordering::Less,
            (x, y, _, _) if x > y => Ordering::Greater,
            (_, _, x, y) if x < y => Ordering::Less,
            (_, _, x, y) if x > y => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
    let mut k = 1;
    let mut swap_rank: Vec<usize> = (0..n).collect();
    while k < n {
        sa.sort_by(|&i, &j| compare(i, j, k, &rank));
        swap_rank[sa[0]] = 0;
        for i in 1..n {
            swap_rank[sa[i]] = swap_rank[sa[i - 1]]
                + if compare(sa[i], sa[i - 1], k, &rank) == Ordering::Equal {
                    0
                } else {
                    1
                };
        }
        for i in 0..n {
            rank[i] = swap_rank[i];
        }
        k *= 2;
    }
    sa
}

#[cfg(test)]
mod test_suffix_array {
    use crate::string::suffix_array::*;

    #[test]
    fn it_works() {
        assert_eq!(suffix_array(&vec![1, 2, 3]), vec![0, 1, 2]);
        assert_eq!(suffix_array(&vec![3, 2, 1]), vec![2, 1, 0]);
        assert_eq!(suffix_array(&vec![1, 3, 2, 1]), vec![3, 0, 2, 1]);
        assert_eq!(
            suffix_array(&"abracadabra".chars().collect::<Vec<_>>()),
            vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]
        );
    }
}
