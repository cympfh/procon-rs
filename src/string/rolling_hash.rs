/// String - RollingHash, Rabin-Karp Method
pub struct RollingHashForest {
    hashes: Vec<RollingHash>,
}
impl RollingHashForest {
    pub fn new(xs: &[u64]) -> Self {
        let hashes = vec![
            RollingHash::new(&xs, 10_007, 1_000_000_007),
            RollingHash::new(&xs, 10_007, 810_810_811),
            RollingHash::new(&xs, 1_000_000_007, 1 << 32),
        ];
        Self { hashes }
    }
    pub fn hash(&self, range: std::ops::Range<usize>) -> Vec<u64> {
        self.hashes.iter().map(|h| h.hash(range.clone())).collect()
    }
}

struct RollingHash {
    _base: u64,
    modulo: u64,
    base_pows: Vec<u64>,
    acc_hash: Vec<u64>,
}
impl RollingHash {
    fn new(xs: &[u64], base: u64, modulo: u64) -> Self {
        let n = xs.len();
        let mut base_pows = vec![1];
        let mut acc_hash = vec![0];
        for i in 0..n {
            base_pows.push((base_pows[i] * base) % modulo);
            acc_hash.push(((acc_hash[i] * base) % modulo + xs[i]) % modulo);
        }
        Self {
            _base: base,
            modulo,
            base_pows,
            acc_hash,
        }
    }
    fn hash(&self, range: std::ops::Range<usize>) -> u64 {
        let sup = self.acc_hash[range.end];
        let sub =
            (self.acc_hash[range.start] * self.base_pows[range.end - range.start]) % self.modulo;
        if sup >= sub {
            sup - sub
        } else {
            self.modulo - ((sub - sup) % self.modulo)
        }
    }
}

#[cfg(test)]
mod test_rolling_hash {

    use crate::string::rolling_hash::*;

    #[test]
    fn test_simple_rolling_hash() {
        let hasher = RollingHash::new(&vec![1, 1, 2, 2, 3], 10, 1_000);
        assert_eq!(hasher.hash(0..0), 0);
        assert_eq!(hasher.hash(0..1), 1);
        assert_eq!(hasher.hash(0..2), 11);
        assert_eq!(hasher.hash(0..3), 112);
        assert_eq!(hasher.hash(0..4), 122);
        assert_eq!(hasher.hash(0..5), 223);
        assert_eq!(hasher.hash(1..2), 1);
        assert_eq!(hasher.hash(1..3), 12);
        assert_eq!(hasher.hash(1..4), 122);
    }

    #[test]
    fn test_string_eq() {
        //                     0  1  2  3  4  5  6  7  8
        let s: Vec<u64> = vec![1, 1, 2, 3, 3, 1, 2, 3, 1];
        let h = RollingHashForest::new(&s);
        assert_eq!(h.hash(0..1), h.hash(1..2)); // [1]
        assert_ne!(h.hash(0..2), h.hash(1..3)); // [1, 1] != [1, 2]
        assert_eq!(h.hash(1..4), h.hash(5..8)); // [1, 2, 3]
        assert_eq!(h.hash(4..6), h.hash(7..9)); // [3, 1]
    }
}
