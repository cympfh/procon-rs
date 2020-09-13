/// Natural Numbers - Minimal Exclusion number, mex
pub fn mex(xs: &Vec<usize>) -> usize {
    let mut memo = vec![false; xs.len() + 1];
    for &x in xs.iter() {
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
    }
}
