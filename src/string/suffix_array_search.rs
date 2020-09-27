/// String - Search via SuffixArray
use crate::string::suffix_array::*;

pub fn sa_search<T: Eq + Ord>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    if n < m {
        return None;
    }
    fn substr<T>(range: std::ops::Range<usize>, text: &[T]) -> &[T] {
        if range.start >= text.len() {
            &text[0..0]
        } else if range.end < text.len() {
            &text[range]
        } else {
            &text[range.start..]
        }
    }
    let sa = suffix_array(&text);
    if substr(sa[0]..sa[0] + m, &text) == pattern {
        return Some(0);
    }
    let mut left = 0; // text[left..left+m] <= pattern
    let mut right = n; // >= pattern
    for _ in 0..100 {
        let mid = (left + right) / 2;
        if substr(sa[mid]..sa[mid] + m, &text) <= pattern {
            left = mid;
        } else {
            right = mid;
        }
    }
    if substr(sa[left]..sa[left] + m, &text) == pattern {
        Some(sa[left])
    } else {
        None
    }
}

#[cfg(test)]
mod test_suffix_array_search {
    use crate::string::suffix_array_search::*;

    #[test]
    fn search_uniq() {
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"abrac".chars().collect::<Vec<_>>()
            ),
            Some(0)
        );
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"raca".chars().collect::<Vec<_>>()
            ),
            Some(2)
        );
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"d".chars().collect::<Vec<_>>()
            ),
            Some(6)
        );
    }

    #[test]
    fn search_none() {
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"abrax".chars().collect::<Vec<_>>()
            ),
            None
        );
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"xy".chars().collect::<Vec<_>>()
            ),
            None
        );
        assert_eq!(
            sa_search(
                &"abracadabra".chars().collect::<Vec<_>>(),
                &"x".chars().collect::<Vec<_>>()
            ),
            None
        );
    }

    #[test]
    fn search_multiple() {
        let res = sa_search(
            &"abracadabra".chars().collect::<Vec<_>>(),
            &"abra".chars().collect::<Vec<_>>(),
        );
        assert!(res == Some(0) || res == Some(7));
    }
}
