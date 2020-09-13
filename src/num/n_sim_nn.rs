/// Natural Numbers - Theorem. N sim N^2
use crate::num::base::*;

pub fn n_to_nn<N: Nat>(n: N) -> (N, N) {
    let mut i = N::zero();
    let mut n = n;
    while n > i {
        i += N::one();
        n -= i;
    }
    (n, i - n)
}

pub fn nn_to_n<N: Nat>(a: N, b: N) -> N {
    let two = N::one() + N::one();
    (a + b) * (a + b + N::one()) / two + a
}

#[cfg(test)]
mod test_binom_modint {
    use crate::num::n_sim_nn::*;

    #[test]
    fn test_n_to_nn() {
        assert_eq!(n_to_nn(0_u64), (0, 0));
        assert_eq!(n_to_nn(1_u64), (0, 1));
        assert_eq!(n_to_nn(2_u64), (1, 0));
        assert_eq!(n_to_nn(3_u64), (0, 2));
        assert_eq!(n_to_nn(4_u64), (1, 1));
        assert_eq!(n_to_nn(5_u64), (2, 0));
    }

    #[test]
    fn test_nn_to_n() {
        assert_eq!(nn_to_n(0, 0), (0_u64));
        assert_eq!(nn_to_n(0, 1), (1_u64));
        assert_eq!(nn_to_n(1, 0), (2_u64));
        assert_eq!(nn_to_n(0, 2), (3_u64));
        assert_eq!(nn_to_n(1, 1), (4_u64));
        assert_eq!(nn_to_n(2, 0), (5_u64));
    }
}
