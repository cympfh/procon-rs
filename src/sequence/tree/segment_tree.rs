/// Sequence - Segment Tree
use crate::algebra::monoid::*;

#[derive(Debug, Clone)]
pub struct SegmentTree<X> {
    length: usize,       // of leaves
    length_upper: usize, // power of 2
    size: usize,         // of nodes
    data: Vec<X>,
}
impl<X> std::ops::Index<usize> for SegmentTree<X> {
    type Output = X;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}
impl<X: Copy + Monoid> SegmentTree<X> {
    pub fn new(length: usize) -> Self {
        let mut length_upper = 1;
        while length_upper < length {
            length_upper *= 2;
        }
        let size = length_upper * 2 - 1;
        let data = vec![X::unit(); size];
        SegmentTree {
            length,
            length_upper,
            size,
            data,
        }
    }
    pub fn from(xs: Vec<X>) -> Self {
        let mut tree = Self::new(xs.len());
        for i in 0..xs.len() {
            tree.data[tree.size / 2 + i] = xs[i];
        }
        for i in (0..tree.size / 2).rev() {
            tree.data[i] = tree.data[2 * i + 1] * tree.data[2 * i + 2];
        }
        tree
    }
    pub fn to_vec(self) -> Vec<X> {
        self.data[self.size / 2..].to_vec()
    }
    pub fn update(&mut self, i: usize, t: X) {
        let mut u = self.size / 2 + i;
        self.data[u] = t;
        while u > 0 {
            u = (u - 1) / 2;
            self.data[u] = self.data[u * 2 + 1] * self.data[u * 2 + 2];
        }
    }
    fn product_sub(
        &self,
        range: std::ops::Range<usize>,
        u: usize,
        focus: std::ops::Range<usize>,
    ) -> X {
        if focus.end <= range.start || range.end <= focus.start {
            X::unit()
        } else if range.start <= focus.start && focus.end <= range.end {
            self.data[u]
        } else {
            let mid = (focus.start + focus.end) / 2;
            let a = self.product_sub(range.clone(), u * 2 + 1, focus.start..mid);
            let b = self.product_sub(range.clone(), u * 2 + 2, mid..focus.end);
            a * b
        }
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> X {
        self.product_sub(range, 0, 0..self.length_upper)
    }
}
impl<X: std::fmt::Debug> SegmentTree<X> {
    fn debug(&self) {
        for i in 0..self.size {
            if i > 0 && (i + 1).count_ones() == 1 {
                eprintln!();
            }
            eprint!("{:?} ", &self.data[i]);
        }
        eprintln!();
    }
}

#[cfg(test)]
mod test_rmq {
    use crate::sequence::tree::segment_tree::*;

    #[test]
    fn sum() {
        let mut v = vec![1, 2, 3, 4];
        let mut st = SegmentTree::from(v.iter().map(|&x| Sum(x)).collect());
        assert_eq!(st.product(0..0), Sum(0));
        assert_eq!(st.product(0..1), Sum(1));
        assert_eq!(st.product(0..2), Sum(3));
        assert_eq!(st.product(0..3), Sum(6));
        assert_eq!(st.product(0..4), Sum(10));
        assert_eq!(st.product(1..4), Sum(9));
        assert_eq!(st.product(2..4), Sum(7));
        assert_eq!(st.product(3..4), Sum(4));

        st.update(1, Sum(-2)); //[1,-2,3,4]
        assert_eq!(st.product(0..0), Sum(0));
        assert_eq!(st.product(0..1), Sum(1));
        assert_eq!(st.product(0..2), Sum(-1));
        assert_eq!(st.product(0..3), Sum(2));
        assert_eq!(st.product(0..4), Sum(6));
    }
}
