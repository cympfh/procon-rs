/// String Serach - Shift-And Algorithm
pub fn shift_and(text: &str, pattern: &str) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    assert!(m <= 128);
    let text_chars: Vec<usize> = text.chars().map(|c| c as usize).collect();
    let pattern_chars: Vec<usize> = pattern.chars().map(|c| c as usize).collect();
    let g = 1 << (m - 1);
    // DP
    let mut d = [0_u128; 300];
    for i in 0..m {
        d[pattern_chars[i]] ^= 1 << i
    }
    let mut x = 0;
    for i in 0..n {
        x = (1 | (x << 1)) & d[text_chars[i]];
        if x & g > 0 {
            return Some(i + 1 - m);
        }
    }
    return None;
}

#[cfg(test)]
mod test_shiftand {
    use crate::string::shiftand::*;

    #[test]
    fn it_works() {
        assert_eq!(shift_and(&"abcdefghijklmnopqrstuvwxyz", &"abc"), Some(0));
        assert_eq!(shift_and(&"abcdefghijklmnopqrstuvwxyz", &"z"), Some(25));
        assert_eq!(shift_and(&"abcdefghijklmnopqrstuvwxyz", &"zz"), None);
        assert_eq!(
            shift_and(&"abcdefghijklmnopqrstuvwxyz", &"abcdefghijklmnopqrstuvwxyz"),
            Some(0)
        );
        assert_eq!(
            shift_and(
                &"abcdefghijklmnopqrstuvwxyz",
                &"abcdefghijklmnopqrstuvwxyzz"
            ),
            None
        );
    }

    #[test]
    fn long_text_long_pattern() {
        let mut text = String::new();
        let mut pattern = String::new();
        for _ in 0..200 {
            text.push('a');
        }
        for _ in 0..128 {
            pattern.push('a');
        }
        assert_eq!(shift_and(&text, &pattern), Some(0));
    }
}
