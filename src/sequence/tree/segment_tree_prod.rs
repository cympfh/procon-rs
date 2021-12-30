/// Sequence - Segment Tree of Prod
use crate::algebra::monoid::*;
use crate::sequence::tree::segment_tree::*;

pub type SegmentTreeProd = SegmentTree<i64>;

#[cfg(test)]
mod test_segment_tree_prod {
    use crate::sequence::tree::segment_tree_prod::*;

    #[test]
    fn test_segment_tree_prod() {
        let mut v = vec![1, 2, 3, 4];
        let mut st = SegmentTreeProd::from(v.clone());

        // assert with naiive prod
        for i in 0..v.len() {
            for j in i..v.len() {
                let mut naiive_prod = 1;
                for k in i..j {
                    naiive_prod *= v[k];
                }
                assert_eq!(st.product(i..j), naiive_prod);
            }
        }

        st.update(1, 0);
        v[1] = 0;

        // assert with naiive prod
        for i in 0..v.len() {
            for j in i..v.len() {
                let mut naiive_prod = 1;
                for k in i..j {
                    naiive_prod *= v[k];
                }
                assert_eq!(st.product(i..j), naiive_prod);
            }
        }
    }
}
