/// Number - Binomial Coefficient on ModInt
use crate::algebra::modint::*;
pub fn binom(n: ModInt, k: ModInt) -> ModInt {
    if k.0 == 0 {
        ModInt(1, n.1)
    } else if n.0 < k.0 * 2 {
        binom(n, n - k)
    } else {
        let mut c = ModInt(1, n.1);
        for i in 0..k.0 {
            c *= n - i;
            c /= k - i;
        }
        c
    }
}

#[cfg(test)]
mod test_binom_modint {
    use crate::num::binom_modint::*;

    #[test]
    fn it_works() {
        const MOD: i64 = 1000000007;
        assert_eq!(binom(ModInt(5, MOD), ModInt(0, MOD)).unwrap(), 1);
        assert_eq!(binom(ModInt(5, MOD), ModInt(1, MOD)).unwrap(), 5);
        assert_eq!(binom(ModInt(5, MOD), ModInt(2, MOD)).unwrap(), 10);
        assert_eq!(binom(ModInt(5, MOD), ModInt(3, MOD)).unwrap(), 10);
        assert_eq!(binom(ModInt(5, MOD), ModInt(4, MOD)).unwrap(), 5);
        assert_eq!(binom(ModInt(5, MOD), ModInt(5, MOD)).unwrap(), 1);
    }

    #[test]
    fn large_numbers() {
        const MOD: i64 = 107;
        assert_eq!(binom(ModInt(100, MOD), ModInt(50, MOD)).unwrap(), 35);
    }
}
