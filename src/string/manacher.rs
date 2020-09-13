/// String - Test Palindric - Manacher Algorithm
pub fn manacher<A: Ord>(s: &Vec<A>) -> Vec<usize> {
    let n = s.len();
    let mut r = vec![0; 2 * n - 1];
    let mut i = 0;
    let mut j = 0;
    while i < 2 * n - 1 {
        while i >= j && i + j + 1 < 2 * n && s[(i - j) / 2] == s[(i + j + 1) / 2] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i + k < 2 * n - 1 && i >= k && r[i] >= k && r[i - k] != r[i] - k {
            r[i + k] = std::cmp::min(r[i - k], r[i] - k);
            k += 1;
        }
        i += k;
        j = if j > k { j - k } else { 0 }
    }
    r
}

#[cfg(test)]
mod test_manacher {
    use crate::string::manacher::*;

    #[test]
    fn it_works() {
        assert_eq!(manacher(&"a".chars().collect()), vec![1]);
        assert_eq!(manacher(&"ab".chars().collect()), vec![1, 0, 1]);
        assert_eq!(manacher(&"aba".chars().collect()), vec![1, 0, 3, 0, 1]);
        assert_eq!(manacher(&"abc".chars().collect()), vec![1, 0, 1, 0, 1]);
    }
}
