/// Sequence - Binary Indexed Tree (Fenwick Tree)
use crate::algebra::monoid::*;

struct BIT<T> {
    size: usize,
    array: Vec<T>,
}
impl<T: Copy + Monoid> BIT<T> {
    fn new(n: usize) -> BIT<T> {
        BIT {
            size: n,
            array: vec![T::unit(); n + 1],
        }
    }
    fn add(&mut self, idx: usize, w: T) {
        let mut x = idx + 1;
        while x <= self.size {
            self.array[x] = self.array[x] * w;
            let xi = x as i32;
            x += (xi & -xi) as usize;
        }
    }
    /// sum of [0, idx)
    fn sum_up(&self, idx: usize) -> T {
        let mut sum = T::unit();
        let mut x = idx;
        while x > 0 {
            sum = sum * self.array[x];
            let xi = x as i32;
            x -= (xi & -xi) as usize;
        }
        sum
    }
}

#[cfg(test)]
mod test_bit {
    use crate::algebra::monoid::Sum;
    use crate::sequence::interval_tree::bit::*;

    #[test]
    fn sum() {
        let mut bit = BIT::new(10);
        for i in 0..10 {
            bit.add(i, Sum(i as i64));
        }
        let mut ac = 0;
        for i in 0..11 {
            assert_eq!(bit.sum_up(i), Sum(ac));
            ac = ac + i as i64;
        }
    }

    #[test]
    fn prod() {
        let mut bit = BIT::new(10);
        for i in 0..10 {
            bit.add(i, Prod((i + 1) as i64));
        }
        let mut ac = 1;
        for i in 0..11 {
            assert_eq!(bit.sum_up(i), Prod(ac));
            ac = ac * (i + 1) as i64;
        }
    }
}
