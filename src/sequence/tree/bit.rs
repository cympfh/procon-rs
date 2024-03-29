use crate::algebra::group_additive::*;

/// Sequence - Binary Indexed Tree (Fenwick Tree) of Additive Group (+, 0)
pub struct BIT<X> {
    size: usize,
    array: Vec<X>,
}
impl<X: Copy + AGroup> BIT<X> {
    pub fn new(size: usize) -> Self {
        BIT {
            size,
            array: vec![X::zero(); size + 1],
        }
    }
    pub fn from(xs: &Vec<X>) -> Self {
        let n = xs.len();
        let mut r = Self::new(n);
        for i in 0..n {
            r.add(i, xs[i]);
        }
        r
    }
    pub fn add(&mut self, idx: usize, w: X) {
        let mut x = idx + 1;
        while x <= self.size {
            self.array[x] = self.array[x] + w;
            let xi = x as i32;
            x += (xi & -xi) as usize;
        }
    }
    /// sum of [0, idx)
    pub fn sum_up(&self, idx: usize) -> X {
        let mut sum = X::zero();
        let mut x = idx;
        while x > 0 {
            sum = sum + self.array[x];
            let xi = x as i32;
            x -= (xi & -xi) as usize;
        }
        sum
    }
    /// sum of [left, right)
    pub fn sum(&self, range: std::ops::Range<usize>) -> X {
        if range.end <= range.start {
            return X::zero();
        }
        self.sum_up(range.end) - self.sum_up(range.start)
    }
}

#[cfg(test)]
mod test_bit {
    use crate::sequence::tree::bit::*;

    #[test]
    fn sum() {
        let mut bit = BIT::new(10);
        for i in 0..10 {
            bit.add(i, i as i64);
        }
        let mut ac = 0;
        for i in 0..11 {
            assert_eq!(bit.sum_up(i), ac);
            ac = ac + i as i64;
        }
    }
}
