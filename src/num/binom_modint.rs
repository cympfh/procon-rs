/// Number - Binomial Coefficient on ModInt
use crate::algebra::modint::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binom {
    n: u64,
    k: u64,
    coeff: ModInt,
}

impl Binom {
    pub fn unwrap(&self) -> ModInt {
        self.coeff
    }

    /// Calc Binom-Coeff with O(k)
    pub fn new(n: u64, k: u64, modulo: i64) -> Self {
        if k == 0 {
            let coeff = ModInt(1, modulo);
            Self { n, k, coeff }
        } else if n < k || n == 0 {
            let coeff = ModInt(0, modulo);
            Self { n, k, coeff }
        } else if n < k * 2 {
            let mut m = Self::new(n, n - k, modulo);
            m.k = k;
            m
        } else {
            let mut c = ModInt(1, modulo);
            for i in 0..k {
                c *= (n - i) as i64;
                c /= (k - i) as i64;
            }
            Self { n, k, coeff: c }
        }
    }

    /// Calc `binom(n, k)` with a Hint
    pub fn new_with_hint(n: u64, k: u64, hint: &Binom) -> Self {
        if k == 0 {
            let coeff = ModInt(1, hint.unwrap().1);
            return Self { n, k, coeff };
        }
        if n < k || n == 0 {
            let coeff = ModInt(0, hint.unwrap().1);
            return Self { n, k, coeff };
        }
        if n == hint.n && k == hint.k {
            return *hint;
        }
        let (n_next, k_next, c_next) = if n < hint.n && k < hint.k {
            let c = hint.unwrap() * hint.k as i64 / hint.n as i64;
            (hint.n - 1, hint.k - 1, c)
        } else if n > hint.n && k > hint.k {
            let c = hint.unwrap() * (hint.n + 1) as i64 / (hint.k + 1) as i64;
            (hint.n + 1, hint.k + 1, c)
        } else if n > hint.n {
            let c = hint.unwrap() * (hint.n + 1) as i64 / (hint.n - hint.k + 1) as i64;
            (hint.n + 1, hint.k, c)
        } else if n < hint.n {
            let c = hint.unwrap() * (hint.n - hint.k) as i64 / hint.n as i64;
            (hint.n - 1, hint.k, c)
        } else if k > hint.k {
            let c = hint.unwrap() * (hint.n - hint.k) as i64 / (hint.k + 1) as i64;
            (hint.n, hint.k + 1, c)
        } else {
            let c = hint.unwrap() * hint.k as i64 / (hint.n - hint.k + 1) as i64;
            (hint.n, hint.k - 1, c)
        };
        let nexthint = Binom {
            n: n_next,
            k: k_next,
            coeff: c_next,
        };
        Self::new_with_hint(n, k, &nexthint)
    }
}

#[cfg(test)]
mod test_binom_modint {
    use crate::num::binom_modint::*;

    #[test]
    fn it_works() {
        const MOD: i64 = 1000000007;
        assert_eq!(Binom::new(5, 0, MOD).unwrap().unwrap(), 1);
        assert_eq!(Binom::new(5, 1, MOD).unwrap().unwrap(), 5);
        assert_eq!(Binom::new(5, 2, MOD).unwrap().unwrap(), 10);
        assert_eq!(Binom::new(5, 3, MOD).unwrap().unwrap(), 10);
        assert_eq!(Binom::new(5, 4, MOD).unwrap().unwrap(), 5);
        assert_eq!(Binom::new(5, 5, MOD).unwrap().unwrap(), 1);
    }

    #[test]
    fn large_numbers() {
        const MOD: i64 = 107;
        assert_eq!(Binom::new(100, 50, MOD).unwrap().unwrap(), 35);
    }

    #[test]
    fn test_with_hint() {
        const MOD: i64 = 107;
        let c = Binom::new(5, 2, MOD);
        assert_eq!(Binom::new_with_hint(4, 2, &c), Binom::new(4, 2, MOD));
        for n in 3..8 {
            for k in 0..=n {
                assert_eq!(Binom::new_with_hint(n, k, &c), Binom::new(n, k, MOD));
            }
        }
    }

    #[test]
    fn test_erroneous() {
        const MOD: i64 = 107;
        assert_eq!(
            Binom::new(0, 0, MOD),
            Binom {
                n: 0,
                k: 0,
                coeff: ModInt(1, MOD)
            }
        );
        assert_eq!(
            Binom::new(0, 1, MOD),
            Binom {
                n: 0,
                k: 1,
                coeff: ModInt(0, MOD)
            }
        );
        assert_eq!(
            Binom::new(1, 2, MOD),
            Binom {
                n: 1,
                k: 2,
                coeff: ModInt(0, MOD)
            }
        );
        assert_eq!(
            Binom::new_with_hint(1, 2, &Binom::new(1, 1, MOD)),
            Binom {
                n: 1,
                k: 2,
                coeff: ModInt(0, MOD)
            }
        );
    }
}
