use crate::algorithm::binary_search::*;

/// Count elements holding the condition
/// the condition has monotone: false -> true
pub fn count_if<X: Complete>(xs: &Vec<X>, cond: &dyn Fn(X) -> bool) -> usize {
    let n = xs.len();
    if n == 0 {
        return 0;
    }
    let lb = lowerbound(0..n, &|i| cond(xs[i])).unwrap_or(n);
    n - lb
}

#[cfg(test)]
mod test_count {
    use crate::algorithm::binary_search_count_if::count_if;
    #[test]
    fn test_count_if() {
        let v: Vec<i64> = vec![1, 4, 9, 16, 25];
        assert_eq!(count_if(&v, &|x| x > 0), 5);
        assert_eq!(count_if(&v, &|x| x >= 9), 3);
        assert_eq!(count_if(&v, &|x| x >= 25), 1);
        assert_eq!(count_if(&v, &|x| x > 100), 0);
        assert_eq!(count_if(&v, &|x| x * x > 20), 3);
    }
    #[test]
    fn test_count_if_empty() {
        let v: Vec<i64> = vec![];
        assert_eq!(count_if(&v, &|x| x < 10 || x >= 10), 0);
    }
}
