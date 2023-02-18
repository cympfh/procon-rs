/// Number - GCD on Natural Numbers
pub fn gcd(a: i128, b: i128) -> i128 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 {
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
        assert_eq!(gcd(10, 15), 5);
        assert_eq!(gcd(-10, 15), 5);
        assert_eq!(gcd(10, -15), 5);
        assert_eq!(gcd(-10, -15), 5);
        assert_eq!(gcd(-5, 0), 5);
    }
}
