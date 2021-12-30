/// Sequence - Segment Tree of Sum
use crate::algebra::monoid_sum::*;
use crate::sequence::tree::segment_tree::*;

pub struct SegmentTreeSum {
    pub t: SegmentTree<Sum>,
}
impl SegmentTreeSum {
    pub fn new(size: usize) -> Self {
        let t = SegmentTree::new(size);
        Self { t }
    }
    pub fn from(xs: Vec<i64>) -> Self {
        let t = SegmentTree::from(xs.iter().map(|&x| Sum(x)).collect());
        Self { t }
    }
    pub fn to_vec(self) -> Vec<i64> {
        self.t.to_vec().iter().map(|sm| sm.0).collect()
    }
    pub fn update(&mut self, i: usize, x: i64) {
        self.t.update(i, Sum(x));
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> i64 {
        self.t.product(range).0
    }
}

#[cfg(test)]
mod test_segment_tree_sum {
    use crate::sequence::tree::segment_tree_sum::*;

    #[test]
    fn test_segment_tree_sum() {
        let mut v = vec![1, 2, 3, 4];
        let mut st = SegmentTreeSum::from(v.clone());

        // assert with naiive sum
        for i in 0..v.len() {
            for j in i..v.len() {
                let mut naiive_sum = 0;
                for k in i..j {
                    naiive_sum += v[k];
                }
                assert_eq!(st.product(i..j), naiive_sum);
            }
        }

        st.update(1, -2);
        v[1] = -2;

        // assert with naiive sum
        for i in 0..v.len() {
            for j in i..v.len() {
                let mut naiive_sum = 0;
                for k in i..j {
                    naiive_sum += v[k];
                }
                assert_eq!(st.product(i..j), naiive_sum);
            }
        }
    }
}
