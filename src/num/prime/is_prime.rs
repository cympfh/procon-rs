/// Prime Numbers - Is a prime number?
use crate::num::prime::mrtest::*;

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        true
    } else if n < 2 || n % 2 == 0 {
        false
    } else {
        for p in 2..10_000_000 {
            if p * p > n {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }
        mrtest(n)
    }
}

#[cfg(test)]
mod test_is_prime {
    use crate::num::prime::is_prime::is_prime;

    #[test]
    fn it_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
        let not_primes = vec![0, 1, 4, 6, 8, 9, 12, 15, 21, 25, 27, 33, 35];
        for p in primes {
            assert!(is_prime(p));
        }
        for q in not_primes {
            assert!(!is_prime(q));
        }
    }
}
