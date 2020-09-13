/// Prime Numbers - Miller Rabin Test
use crate::num::random::xorshift::*;

pub fn mrtest(n: u64) -> bool {
    fn powmod(a: u64, b: u64, m: u64) -> u64 {
        if b == 0 {
            1
        } else if b == 1 {
            a % m
        } else {
            let r = powmod((a * a) % m, b / 2, m);
            if b % 2 == 0 {
                r
            } else {
                (r * a) % m
            }
        }
    }
    if n < 2 {
        false
    } else if n < 4 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut rand = XorShift::new();
        let mut d = n - 1;
        while d % 2 == 0 {
            d /= 2
        }
        for _ in 0..100 {
            let a = rand.gen::<u64>() % (n - 1) + 1;
            let mut y = powmod(a, d, n);
            let mut t = d;
            while t != n - 1 && y != 1 && y != n - 1 {
                y = (y * y) % n;
                t <<= 1;
            }
            if y != n - 1 && t % 2 == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test_mrtest {
    use crate::num::prime::mrtest::*;

    #[test]
    fn it_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
        let not_primes = vec![0, 1, 4, 6, 8, 9, 12, 15, 21, 25, 27, 33, 35];
        for p in primes {
            assert!(mrtest(p));
        }
        for q in not_primes {
            assert!(!mrtest(q));
        }
    }
}
