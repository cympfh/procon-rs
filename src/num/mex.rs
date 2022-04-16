/// Natural Numbers - Minimal Exclusion number, mex
pub fn mex(xs: &Vec<usize>) -> usize {
    let m = xs.len();
    let mut memo = vec![false; m + 1];
    for &x in xs.iter().filter(|&x| x <= &m) {
        memo[x] = true;
    }
    memo.iter().take_while(|&&b| b).count()
}

#[cfg(test)]
mod test_binom_modint {
    use crate::num::mex::*;

    #[test]
    fn it_works() {
        assert_eq!(mex(&vec![1, 2, 3]), 0);
        assert_eq!(mex(&vec![0, 2, 3]), 1);
        assert_eq!(mex(&vec![0, 1, 3]), 2);
        assert_eq!(mex(&vec![0, 1000]), 1);
    }
}
