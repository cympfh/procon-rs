/// Number - Binomial Coefficient - Pascal's Triangle
use crate::algebra::ring::*;
pub fn pascal_triagle<R: Ring + Copy>(n: usize) -> Vec<Vec<R>> {
    let mut binom = vec![vec![R::zero(); n + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 0..i + 1 {
            binom[i][j] = {
                if i == 1 || j == 0 {
                    R::one()
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
    fn test_i64() {
        let pascal = pascal_triagle::<i64>(5);
        assert_eq!(pascal[5][0], 1);
        assert_eq!(pascal[5][1], 5);
        assert_eq!(pascal[5][2], 10);
        assert_eq!(pascal[5][3], 10);
        assert_eq!(pascal[5][4], 5);
        assert_eq!(pascal[5][5], 1);
    }

    #[test]
    fn test_modint() {
        use crate::algebra::modint::*;
        use crate::mint;
        let pascal = pascal_triagle::<ModInt>(5);
        assert_eq!(pascal[5][0], mint!(1));
        assert_eq!(pascal[5][1], mint!(5));
        assert_eq!(pascal[5][2], mint!(10));
        assert_eq!(pascal[5][3], mint!(10));
        assert_eq!(pascal[5][4], mint!(5));
        assert_eq!(pascal[5][5], mint!(1));
    }
}
