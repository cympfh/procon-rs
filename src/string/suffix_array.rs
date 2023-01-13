/// String - Suffix Array (Manber&Myers, O(n (log n)^2))
pub fn suffix_array<T: Eq + Ord>(s: &[T]) -> Vec<usize> {
    use std::collections::{BTreeMap, BTreeSet};
    let n = s.len();
    if n <= 1 {
        return (0..n).collect();
    }
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = (0..n).collect();
    {
        let alphabet: BTreeSet<&T> = s.iter().collect();
        let chr: BTreeMap<_, usize> = alphabet
            .iter()
            .enumerate()
            .map(|(idx, c)| (c, idx))
            .collect();
        for i in 0..n {
            rank[i] = chr[&&&s[i]];
        }
    }
    fn key(i: usize, k: usize, rank: &Vec<usize>) -> (usize, Option<&usize>) {
        (rank[i], rank.get(i + k))
    }
    fn eq(i: usize, j: usize, k: usize, rank: &Vec<usize>) -> bool {
        key(i, k, rank) == key(j, k, rank)
    }
    let mut k = 1;
    while k < n {
        let mut alt: Vec<usize> = (0..n).collect();
        sa.sort_by_key(|&i| key(i, k, &rank));
        alt[sa[0]] = 0;
        for i in 1..n {
            alt[sa[i]] = alt[sa[i - 1]] + if eq(sa[i], sa[i - 1], k, &rank) { 0 } else { 1 };
        }
        rank = alt;
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
