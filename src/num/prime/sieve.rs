/// Prime Numbers - Sieve of Eratosthenes
pub fn prime_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;
    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }
    s
}

#[cfg(test)]
mod test_sieve {
    use crate::num::prime::sieve::*;

    #[test]
    fn it_works() {
        let ps = prime_sieve(1000);
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
        let not_primes = vec![0, 1, 4, 6, 8, 9, 12, 15, 21, 25, 27, 33, 35];
        for p in primes {
            assert!(ps[p]);
        }
        for q in not_primes {
            assert!(!ps[q]);
        }
    }
}
