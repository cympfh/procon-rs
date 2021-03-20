/// Number - Prime Factorization
pub fn factorize(n: u64) -> Vec<(u64, usize)> {
    let mut m = n;
    let mut r = vec![];
    for x in 2..=n {
        if m == 1 {
            break;
        }
        if x * x > n {
            r.push((m, 1));
            break;
        }
        if n % x != 0 {
            continue;
        }
        let mut c = 0;
        while m % x == 0 {
            m /= x;
            c += 1;
        }
        if c > 0 {
            r.push((x, c));
        }
    }
    r
}

#[cfg(test)]
mod test_factorize {
    use crate::num::prime::factorize::*;

    #[test]
    fn it_works() {
        assert_eq!(factorize(1), vec![]);
        assert_eq!(factorize(2), vec![(2, 1)]);
        assert_eq!(factorize(4), vec![(2, 2)]);
        assert_eq!(factorize(1024), vec![(2, 10)]);
        assert_eq!(factorize(1024 * 9), vec![(2, 10), (3, 2)]);
    }
}
