/// Number - Discrete Logarithm
use crate::algebra::modint::*;

// Returns x; pow(a, x) == b
pub fn dlog(a: ModInt, b: ModInt) -> i128 {
    let s = {
        let mut s = 1;
        while s * s <= a.1 {
            s += 1;
        }
        s - 1
    }; // sqrt(a)
    let mut pows = std::collections::HashMap::new();
    {
        let mut pow = ModInt(1, a.1);
        for i in 0..s {
            pows.insert(pow.0, i);
            pow *= a;
        }
    }
    let aa = a.pow(-s);
    let mut ac = b;
    for i in 0..s {
        if pows.contains_key(&ac.0) {
            return pows[&ac.0] + i * s;
        }
        ac *= aa;
    }
    -1
}

#[cfg(test)]
mod test_dlog {
    use crate::num::dlog::*;

    #[test]
    fn it_works() {
        const MOD: i128 = 1_000_000_007;
        assert_eq!(dlog(ModInt(2, MOD), ModInt(1024, MOD)), 10);
    }

    #[test]
    fn under_modulo() {
        const MOD: i128 = 107;
        assert_eq!(dlog(ModInt(2, MOD), ModInt(2, MOD).pow(10)), 10);
    }
}
