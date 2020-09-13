/// Number - GCD on Natural Numbers
use crate::num::base::*;
pub fn gcd<N: Nat>(a: N, b: N) -> N {
    if b == N::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test_gcd {
    use crate::num::gcd::*;

    #[test]
    fn it_works() {
        assert_eq!(gcd(10_usize, 15), 5);
        assert_eq!(gcd(10_u32, 15), 5);
        assert_eq!(gcd(10_u64, 15), 5);
        assert_eq!(gcd(10_u128, 15), 5);
    }
}
