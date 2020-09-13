/// String Compression - Run-Length
pub fn runlength<A: Copy + PartialEq>(xs: &Vec<A>) -> Vec<(A, usize)> {
    let m = xs.len();
    if m == 0 {
        return vec![];
    }
    let mut count = 1;
    let mut result = vec![];
    for i in 0..m {
        if i > 0 {
            if xs[i - 1] == xs[i] {
                count += 1;
            } else {
                count = 1;
            }
        }
        if i == m - 1 || xs[i] != xs[i + 1] {
            result.push((xs[i], count));
        }
    }
    result
}

#[cfg(test)]
mod test_runlength {
    use crate::string::runlength::*;

    #[test]
    fn it_works() {
        assert_eq!(runlength(&"a".chars().collect()), vec![('a', 1)]);
        assert_eq!(runlength(&"aaa".chars().collect()), vec![('a', 3)]);
        assert_eq!(
            runlength(&"aaab".chars().collect()),
            vec![('a', 3), ('b', 1)]
        );
        assert_eq!(
            runlength(&"aaabcc".chars().collect()),
            vec![('a', 3), ('b', 1), ('c', 2)]
        );
    }
}
