/// Number - Binomial Coefficient - Pascal's Triangle
use crate::num::base::*;
pub fn pascal_triagle<N: Nat>(n: usize) -> Vec<Vec<N>> {
    let mut binom = vec![vec![N::zero(); n + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 0..i + 1 {
            binom[i][j] = {
                if i == 1 || j == 0 {
                    N::one()
                } else if j + j > i {
                    binom[i][i - j]
                } else {
                    binom[i - 1][j] + binom[i - 1][j - 1]
                }
            };
        }
    }
    binom
}

#[cfg(test)]
mod test_binom_pascal {
    use crate::num::binom_pascal::*;

    #[test]
    fn it_works() {
        let pascal = pascal_triagle::<u64>(5);
        assert_eq!(pascal[5][0], 1);
        assert_eq!(pascal[5][1], 5);
        assert_eq!(pascal[5][2], 10);
        assert_eq!(pascal[5][3], 10);
        assert_eq!(pascal[5][4], 5);
        assert_eq!(pascal[5][5], 1);
    }
}
