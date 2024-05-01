/// Sequence - Sparse Table
use crate::algebra::monoid::*;

/// X は冪等モノイド (x * x == x)
#[derive(Debug)]
pub struct SparseTable<X: Monoid> {
    data: Vec<Vec<X>>,
}
impl<X: Monoid + Clone + Copy> SparseTable<X> {
    pub fn new(xs: &Vec<X>) -> Self {
        let n = xs.len();
        let k = (0..64).filter(|i| (1 << i) <= n).count();
        let mut data = vec![vec![X::one(); n]; k];
        for i in 0..n {
            data[0][i] = xs[i].clone();
        }
        let mut len = 2;
        for d in 1..k {
            for i in 0..n - len / 2 {
                data[d][i] = data[d - 1][i] * data[d - 1][i + len / 2];
            }
            len <<= 1;
        }
        Self { data }
    }
    pub fn product(&self, r: std::ops::Range<usize>) -> X {
        if r.start >= r.end {
            return X::one();
        }
        let len = r.end - r.start;
        let k = (1..64).filter(|i| (1 << i) <= len).count();
        self.data[k][r.start] * self.data[k][r.end - (1 << k)]
    }
}

#[cfg(test)]
mod test_sparse_table {
    use crate::algebra::monoid_max::*;
    use crate::algebra::monoid_min::*;
    use crate::sequence::tree::sparse_table::*;
    #[test]
    fn test_st_1() {
        let st = SparseTable::new(&vec![MinInt::Val(1)]);
        assert_eq!(st.product(0..1), MinInt::Val(1));
        assert_eq!(st.product(0..0), MinInt::Maximal);
        assert_eq!(st.product(1..1), MinInt::Maximal);
    }
    #[test]
    fn test_st_2() {
        let st = SparseTable::new(&vec![
            MinInt::Val(1),
            MinInt::Val(2),
            MinInt::Val(2),
            MinInt::Val(1),
        ]);
        assert_eq!(st.product(0..4), MinInt::Val(1));
        assert_eq!(st.product(0..2), MinInt::Val(1));
        assert_eq!(st.product(1..3), MinInt::Val(2));
    }
    #[test]
    fn test_st_3() {
        let data = vec![
            MaxInt::Val(1),
            MaxInt::Val(1),
            MaxInt::Val(1),
            MaxInt::Val(100),
            MaxInt::Val(1),
        ];
        let st = SparseTable::new(&data);
        for i in 0..5 {
            for j in i..5 {
                let mut naiiv = MaxInt::Minimal;
                for k in i..j {
                    naiiv = naiiv * data[k];
                }
                assert_eq!(st.product(i..j), naiiv);
            }
        }
    }
}
