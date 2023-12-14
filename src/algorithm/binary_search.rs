/// Algorithm - Binary Search (lowerbound)
pub trait Complete: Copy + PartialEq + PartialOrd {
    fn mid(self, other: Self) -> Self;
}
#[macro_export]
macro_rules! complete {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl Complete for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
complete! { usize, mid(self, other) = { (self + other) / 2 }}
complete! { u128, mid(self, other) = { (self + other) / 2 }}
complete! { i128, mid(self, other) = { (self + other) / 2 }}
complete! { u64, mid(self, other) = { (self + other) / 2 }}
complete! { i64, mid(self, other) = { (self + other) / 2 }}
complete! { f64, mid(self, other) = { (self + other) / 2.0 }}

/// Find a lowerbound for the condition
/// the condition has monotone: false -> true
pub fn lowerbound<T: Complete>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if r.is_empty() {
        return None;
    }
    if cond(r.start) {
        return Some(r.start);
    }
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

    #[test]
    fn search_on_empty() {
        assert_eq!(lowerbound(0_i64..0, &|i| i < 0 || i >= 0), None);
        assert_eq!(lowerbound(1.0..-1.0_f64, &|x| x * x >= 0.0), None);
    }
}
