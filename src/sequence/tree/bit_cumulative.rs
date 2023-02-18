use crate::algebra::group_additive::*;
use crate::sequence::tree::bit::*;

/// Sequence - Cumulative Array by BIT (Fenwick Tree)
pub struct CumBIT<X> {
    data: BIT<X>,
}
impl<X: Copy + AGroup> CumBIT<X> {
    pub fn new(size: usize) -> Self {
        let data = BIT::new(size);
        Self { data }
    }
    pub fn add(&mut self, range: std::ops::Range<usize>, x: X) {
        self.data.add(range.start, x);
        self.data.add(range.end, -x);
    }
    pub fn at(&self, idx: usize) -> X {
        self.data.sum_up(idx + 1)
    }
}

#[cfg(test)]
mod test_cumbit {
    use crate::sequence::tree::bit_cumulative::*;
    macro_rules! assert_veq {
        ($cbit:expr, $arr:expr) => {
            for i in 0..$arr.len() {
                assert_eq!($cbit.at(i), $arr[i]);
            }
        };
    }
    #[test]
    fn it_works() {
        let mut cbit = CumBIT::new(5);
        assert_veq!(cbit, vec![0, 0, 0, 0, 0]);
        cbit.add(1..4, 2_i128);
        assert_veq!(cbit, vec![0, 2, 2, 2, 0]);
        cbit.add(0..5, 1);
        assert_veq!(cbit, vec![1, 3, 3, 3, 1]);
        cbit.add(3..5, -1);
        assert_veq!(cbit, vec![1, 3, 3, 2, 0]);
        cbit.add(1..2, 100);
        assert_veq!(cbit, vec![1, 103, 3, 2, 0]);
    }
}
