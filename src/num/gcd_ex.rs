/// Number - Extended GCD on Integers
/// Solve of a*x + b*y = gcd(x, y)
/// - Args: x, y
/// - Returns: (a, b, gcd(x, y))
use crate::num::base::*;
pub fn gcd_ex<N: Nat>(x: N, y: N) -> (N, N, N) {
    if y == N::zero() {
        (N::one(), N::zero(), x)
    } else {
        let (p, q, g) = gcd_ex(y, x % y);
        (q, p - q * (x / y), g)
    }
}

#[cfg(test)]
mod test_gcd {
    use crate::num::gcd_ex::*;

    #[test]
    fn test_samples() {
        assert_eq!(gcd_ex(3_i64, 6), (1, 0, 3));
        assert_eq!(gcd_ex(3_i64, 1), (0, 1, 1));
    }

    #[test]
    fn test_equality() {
        for x in -4..=4_i64 {
            for y in -4..=4_i64 {
                let (p, q, g) = gcd_ex(x, y);
                assert_eq!(p * x + q * y, g);
            }
        }
    }
}
