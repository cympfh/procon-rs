/// misc - Coodinate Compression (座標圧縮)
pub fn coodinate_compression<T: Clone + Ord>(xs: &Vec<T>) -> Vec<usize> {
    let xset: std::collections::BTreeSet<T> = xs.iter().cloned().collect();
    let xmap: std::collections::BTreeMap<&T, usize> =
        xset.iter().enumerate().map(|(i, x)| (x, i)).collect();
    xs.iter().map(|x| xmap[x]).collect()
}

#[cfg(test)]
mod test_coodinate_compression {
    use crate::misc::coodinate_compression::*;
    #[test]
    fn test_coodinate_compression() {
        assert_eq!(
            coodinate_compression(&vec![1_i64, 2, 3]),
            vec![0_usize, 1, 2]
        );
        assert_eq!(
            coodinate_compression(&vec!['z', 's', 'a']),
            vec![2_usize, 1, 0]
        );
        assert_eq!(coodinate_compression(&vec!["A"]), vec![0]);
    }
    #[test]
    fn test_empty() {
        let v: Vec<u8> = vec![];
        let expected: Vec<usize> = vec![];
        assert_eq!(coodinate_compression(&v), expected);
    }
}
