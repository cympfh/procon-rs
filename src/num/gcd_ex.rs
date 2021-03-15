/// Number - Extended GCD on Integers
/// Solve of a*x + b*y = gcd(x, y)
/// - Args: x, y
/// - Returns: (a, b, gcd(x, y))
use crate::num::base::*;
pub fn gcd_ex<N: Int>(x: N, y: N) -> (N, N, N) {
    let mut r0 = x;
    let mut a0 = N::one();
    let mut b0 = N::zero();
    let mut r = y;
    let mut a = N::zero();
    let mut b = N::one();
    while r > N::zero() {
        let (r2, a2, b2) = (r0 % r, a0 - r0 / r * a, b0 - r0 / r * b);
        r0 = r;
        r = r2;
        a0 = a;
        a = a2;
        b0 = b;
        b = b2;
    }
    (a0, b0, r0)
}

#[cfg(test)]
mod test_gcd {
    use crate::num::gcd_ex::*;

    #[test]
    fn it_works() {
        assert_eq!(gcd_ex(3_i32, 5), (2, -1, 1));
        assert_eq!(gcd_ex(3_i64, 6), (1, 0, 3));
    }
}
