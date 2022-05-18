/// Algorithm - Binary Search (lowerbound)
pub trait CompleteIdx: Copy {
    fn mid(self, other: Self) -> Self;
}
#[macro_export]
macro_rules! completeidx {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl CompleteIdx for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
completeidx! { usize, mid(self, other) = { (self + other) / 2 }}
completeidx! { u128, mid(self, other) = { (self + other) / 2 }}
completeidx! { u64, mid(self, other) = { (self + other) / 2 }}
completeidx! { f64, mid(self, other) = { (self + other) / 2.0 }}

pub fn lowerbound<T: CompleteIdx>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if cond(r.start) {
        return Some(r.start);
    }
    // TODO(from 1.47.0)
    // if r.is_empty() { return None }
    let mut left = r.start;
    let mut right = r.end;
    let mut ok = false;
    for _ in 0..100 {
        let mid = T::mid(left, right);
        if cond(mid) {
            right = mid;
            ok = true;
        } else {
            left = mid;
        }
    }
    if ok {
        Some(right)
    } else {
        None
    }
}

#[cfg(test)]
mod test_binary_search {
    use crate::algorithm::binary_search::*;

    #[test]
    fn search_bound() {
        let v: Vec<i32> = (0..100).collect();
        assert_eq!(lowerbound(0..100, &|i| v[i] > 50), Some(51));
        assert_eq!(lowerbound(0..100, &|i| v[i] >= 0), Some(0));
        assert_eq!(lowerbound(0..100, &|i| v[i] >= 99), Some(99));
        assert_eq!(lowerbound(0..100, &|i| v[i] >= 100), None);
    }

    #[test]
    fn search_on_real_number() {
        let x: f64 = lowerbound(0.0..2.0, &|x| x * x >= 2.0).unwrap();
        assert!((x * x - 2.0).abs() < 0.00001);
    }
}
