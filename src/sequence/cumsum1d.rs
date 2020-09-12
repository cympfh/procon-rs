/// Sequence - Cumulative Summation 1D
use crate::algebra::group::*;

#[derive(Debug)]
pub struct Cumsum1d<T>(Vec<T>);
impl<T: Copy + Group> Cumsum1d<T> {
    pub fn new(xs: &Vec<T>) -> Self {
        let mut ac = T::zero();
        let mut arr = vec![T::zero(); xs.len()];
        for i in 0..arr.len() {
            ac = ac + xs[i];
            arr[i] = ac;
        }
        Self(arr)
    }
    /// sum of [0, idx)
    fn sum_up(&self, idx: usize) -> T {
        if idx > 0 {
            self.0[idx - 1]
        } else {
            T::zero()
        }
    }
    /// sum(i..j) = sum of [i, j)
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        if range.start >= range.end {
            T::zero()
        } else {
            self.sum_up(range.end) - self.sum_up(range.start)
        }
    }
}

#[cfg(test)]
mod test_cumsum {
    use crate::sequence::cumsum1d::*;

    fn naiiv(range: std::ops::Range<usize>, xs: &Vec<i32>) -> i32 {
        range.map(|i| xs[i]).sum()
    }

    fn autocheck(xs: Vec<i32>) {
        let n = xs.len();
        let cs = Cumsum1d::new(&xs);
        for left in 0..n {
            for right in 0..n {
                assert_eq!(cs.sum(left..right), naiiv(left..right, &xs));
            }
        }
    }

    #[test]
    fn test() {
        autocheck(vec![1, 3, 5, 2, 4, 6]);
        autocheck(vec![-1, -2, 0, -3]);
    }
}
