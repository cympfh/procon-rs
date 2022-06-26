use crate::algorithm::binary_search::*;

/// num of elements in the range
pub fn count<X: CompleteIdx + Ord>(xs: &Vec<X>, range: std::ops::Range<X>) -> usize {
    let n = xs.len();
    if n == 0 {
        return 0;
    }
    let rightout = lowerbound(0..n, &|i| xs[i] >= range.end)
        .map(|i| n - i)
        .unwrap_or(0);
    let rightin = lowerbound(0..n, &|i| xs[i] >= range.start)
        .map(|i| n - i)
        .unwrap_or(rightout);
    rightin - rightout
}

#[cfg(test)]
mod test_count {
    use crate::algorithm::binary_search_count::count;
    #[test]
    fn test_count() {
        let v = vec![2_i64, 2, 4, 6, 6, 8];
        assert_eq!(count(&v, 2..6), 3);
        assert_eq!(count(&v, 0..6), 3);
        assert_eq!(count(&v, 0..2), 0);
        assert_eq!(count(&v, 0..3), 2);
        assert_eq!(count(&v, 1..5), 3);
        assert_eq!(count(&v, 1..6), 3);
        assert_eq!(count(&v, 1..100), 6);
        assert_eq!(count(&v, 4..100), 4);
        assert_eq!(count(&v, 4..8), 3);
        assert_eq!(count(&v, 4..9), 4);
        assert_eq!(count(&v, 5..8), 2);
        assert_eq!(count(&v, 5..9), 3);
    }
    #[test]
    fn test_count_empty() {
        let v: Vec<i64> = vec![];
        assert_eq!(count(&v, 2..6), 0);
    }
}
