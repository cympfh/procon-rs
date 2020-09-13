/// Number - Euler's totient function; Phi
pub fn phi(n: u64) -> u64 {
    let mut r = n;
    let mut m = n;
    for p in 2..=n {
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
            }
            r -= r / p;
        }
        if p * p > n {
            break;
        }
    }
    if m > 1 {
        r -= r / m;
    }
    r
}

#[cfg(test)]
mod test_euler_phi {
    use crate::num::euler_phi::*;

    #[test]
    fn test_euler_phi() {
        let truth = vec![
            1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8,
        ];
        for i in 0..truth.len() {
            assert_eq!(phi((i + 1) as u64), truth[i]);
        }

        assert_eq!(phi(5000), 2000);
        assert_eq!(phi(79523), 78960);
        assert_eq!(phi(80000), 32000);
        assert_eq!(phi(99999), 64800);
        assert_eq!(phi(100000), 40000);
    }
}
