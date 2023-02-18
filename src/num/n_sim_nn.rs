/// Natural Numbers - Theorem. N sim N^2
pub fn n_to_nn(n: u128) -> (u128, u128) {
    let mut i = 0;
    let mut n = n;
    while n > i {
        i += 1;
        n -= i;
    }
    (n, i - n)
}

pub fn nn_to_n(a: u128, b: u128) -> u128 {
    (a + b) * (a + b + 1) / 2 + a
}

#[cfg(test)]
mod test_binom_modint {
    use crate::num::n_sim_nn::*;

    #[test]
    fn test_n_to_nn() {
        assert_eq!(n_to_nn(0), (0, 0));
        assert_eq!(n_to_nn(1), (0, 1));
        assert_eq!(n_to_nn(2), (1, 0));
        assert_eq!(n_to_nn(3), (0, 2));
        assert_eq!(n_to_nn(4), (1, 1));
        assert_eq!(n_to_nn(5), (2, 0));
    }

    #[test]
    fn test_nn_to_n() {
        assert_eq!(nn_to_n(0, 0), (0));
        assert_eq!(nn_to_n(0, 1), (1));
        assert_eq!(nn_to_n(1, 0), (2));
        assert_eq!(nn_to_n(0, 2), (3));
        assert_eq!(nn_to_n(1, 1), (4));
        assert_eq!(nn_to_n(2, 0), (5));
    }
}
