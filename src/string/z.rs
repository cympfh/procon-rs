/// String - Z-Algorithm; String Search, Longest Common Prefix
pub fn z<T: Eq>(s: &Vec<T>) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n + 1];
    z[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1
        }
        if j == 0 {
            i += 1;
            continue;
        }
        z[i] = j;
        let mut k = 1;
        while i + k < n && k + z[k] < j {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    z
}

pub fn z_search(text: &str, pattern: &str) -> Option<usize> {
    let m = pattern.len();
    let t = pattern.to_string() + ";$" + text;
    let table = z(&t.chars().collect());
    for i in 0..text.len() {
        if table[i + m + 2] == m {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test_z {

    #[test]
    fn test_z() {
        use crate::string::z::z;
        assert_eq!(z(&"aaa".chars().collect()), vec![3, 2, 1, 0]);
        assert_eq!(
            z(&"abcaabc".chars().collect()),
            vec![7, 0, 0, 1, 3, 0, 0, 0]
        );
    }

    #[test]
    fn test_z_search() {
        use crate::string::z::z_search;
        assert_eq!(z_search("abcaabc", "abc"), Some(0));
        assert_eq!(z_search("abcdefghijklmnopqrstuvwxyz", "abc"), Some(0));
        assert_eq!(z_search("abcdefghijklmnopqrstuvwxyz", "xyz"), Some(23));
        assert_eq!(z_search("abcdefghijklmnopqrstuvwxyz", "xyx"), None);
    }
}
