/// Returns (lambda, mu), Steps before Loop, Length of Loop
pub fn rho<X: Eq + Copy + std::fmt::Debug, F: Fn(X) -> X>(initial_state: X, step: F) -> (u64, u64) {
    let mut x1 = initial_state;
    let mut x2 = initial_state;
    loop {
        x1 = step(x1);
        x2 = step(x2);
        x2 = step(x2);
        if x1 == x2 {
            break;
        }
    }
    let mut lambda = 0;
    let mut x2 = initial_state;
    while x1 != x2 {
        lambda += 1;
        x1 = step(x1);
        x2 = step(x2);
    }
    let mut mu = 0;
    while mu == 0 || x1 != x2 {
        mu += 1;
        x1 = step(x1);
        x2 = step(x2);
        x2 = step(x2);
    }
    (lambda, mu)
}

#[cfg(test)]
mod test_rho {

    use crate::algorithm::rho::*;

    #[test]
    fn test1() {
        // 2 -> 4 -> 6 -> 6 -> ...
        let (lambda, mu) = rho(2, |x| (x * x) % 10);
        assert_eq!((lambda, mu), (2, 1));
    }

    #[test]
    fn test_const() {
        // 1 -> 1 -> ...
        let (lambda, mu) = rho(1, |x| x);
        assert_eq!((lambda, mu), (0, 1));
    }

    #[test]
    fn test2() {
        // 1 -> 0 -> 0 -> ...
        let (lambda, mu) = rho(1, |x| x % 1);
        assert_eq!((lambda, mu), (1, 1));
    }

    #[test]
    fn test3() {
        // 0 -> 1 -> 2 -> 0 -> 1 -> 2 -> 0 -> ...
        let (lambda, mu) = rho(0, |x| (x + 1) % 3);
        assert_eq!((lambda, mu), (0, 3));
    }

    #[test]
    fn test4() {
        // 3 -> 1 -> 2 -> 0 -> 1 -> 2 -> 0 -> ...
        let (lambda, mu) = rho(3, |x| (x + 1) % 3);
        assert_eq!((lambda, mu), (1, 3));
    }

    #[test]
    fn test5() {
        // 0 -> 1 -> 0 -> 1
        let (lambda, mu) = rho(2, |x| 1 - x);
        assert_eq!((lambda, mu), (0, 2));
    }
}
