/// Natural Numbers - Moebius Function
pub fn moebius(n: usize) -> Vec<i64> {
    let mut moe = vec![1; n];
    let mut prime = vec![true; n];
    moe[0] = 0;
    for i in 2..n {
        if !prime[i] {
            continue;
        }
        moe[i] = -1;
        for j in 2..(n - 1) / i + 1 {
            moe[i * j] *= -1;
            prime[i * j] = false;
        }
        for j in 1..(n - 1) / i / i + 1 {
            moe[i * i * j] = 0;
        }
    }
    moe
}

#[cfg(test)]
mod test_moebius {
    use crate::num::moebius::*;

    #[test]
    fn it_works() {
        // https://oeis.org/A008683
        let truth = vec![
            0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0,
            1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1,
            0, -1, 0, 1, 0, 1, 1, -1, 0, -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, -1, 1, 0, 0, 1,
            -1,
        ];
        let moe = moebius(truth.len() + 1);
        for i in 0..truth.len() {
            assert_eq!(moe[i], truth[i]);
        }
    }
}
