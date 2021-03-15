/// Number Theory - Chinese Remainder Theorem (CRT)
/// Solve of x = r[i] mod m[i] => x = y mod z
/// - Args:
///     - rm: &Vec of pair (r[i], m[i])
/// - Returns Some(y, z)
use crate::num::gcd_ex::*;
pub fn crt(rm: &[(i64, i64)]) -> Option<(i64, i64)> {
    assert!(!rm.is_empty());
    for &(_, m) in rm.iter() {
        assert!(m > 0);
    }
    let mut r0 = 0;
    let mut m0 = 1;
    for &(r, m) in rm.iter() {
        let (p, _, d) = gcd_ex(m0, m);
        if (r - r0) % d != 0 {
            return None;
        }
        let tmp = (r - r0) / d * p % (m / d);
        r0 += m0 * tmp;
        m0 *= m / d;
    }
    while r0 < 0 {
        r0 += m0
    }
    Some((r0, m0))
}

#[cfg(test)]
mod test_crt {
    use crate::num::chinese_remainder_theorem::*;
    #[test]
    fn it_works() {
        assert_eq!(crt(&vec![(1, 2)]), Some((1, 2)));
        assert_eq!(crt(&vec![(1, 2), (2, 3)]), Some((5, 6)));
        assert_eq!(crt(&vec![(1, 3), (2, 3)]), None);
        assert_eq!(crt(&vec![(1, 3), (2, 5)]), Some((7, 15)));
        assert_eq!(crt(&vec![(1, 3), (2, 5), (3, 7)]), Some((52, 105)));
        assert_eq!(crt(&vec![(2, 3), (2, 5), (3, 7)]), Some((17, 105)));
        assert_eq!(crt(&vec![(1, 4), (0, 13), (14, 17)]), Some((65, 884)));
        assert_eq!(
            crt(&vec![(1, 4), (1, 8), (0, 13), (14, 17)]),
            Some((65, 1768))
        );
    }
}
