/// Index Compression - zip_index
/// Args:
/// - zs: Vec of Ordering items
/// Returns: (compressed indices, sup)
/// - compressed indices:
///     Vec of usize
///     each elements is in 0..sup
/// - sup:
///     sup of index
pub fn zip_index<T: Clone + Ord>(zs: &[T]) -> (Vec<usize>, usize) {
    let zset: std::collections::BTreeSet<&T> = zs.iter().collect();
    let mut xs: Vec<&T> = zset.iter().copied().collect();
    xs.sort();
    let map: std::collections::BTreeMap<&T, usize> =
        xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let ts = zs
        .iter()
        .map(|z| *map.get(&z).clone().expect("index"))
        .collect();
    (ts, xs.len())
}

#[cfg(test)]
mod test_zip_index {
    use crate::misc::zip_index::*;
    #[test]
    fn it_works() {
        assert_eq!(
            zip_index(&vec![1, 10, 100, 100, 10]),
            (vec![0, 1, 2, 2, 1], 3)
        );
        assert_eq!(
            zip_index(&vec![Some(1), Some(-1), None]),
            (vec![2, 1, 0], 3)
        );
    }
}
