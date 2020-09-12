/// Algorithm - Binary Search
pub trait Integer
where
    Self: std::marker::Sized,
{
    fn close(range: std::ops::Range<Self>) -> bool;
    fn middle(range: std::ops::Range<Self>) -> Self;
}
macro_rules! define_integer {
    ($type:ty, $range:ident, $close_condition:expr, $middle_point:expr) => {
        impl Integer for $type {
            fn close($range: std::ops::Range<Self>) -> bool {
                $close_condition
            }
            fn middle($range: std::ops::Range<Self>) -> Self {
                $middle_point
            }
        }
    };
}
define_integer!(usize, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u32, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u64, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u128, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i32, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i64, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i128, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(
    f32,
    r,
    r.start + 0.00000001 >= r.end,
    (r.start + r.end) / 2.0
);
define_integer!(
    f64,
    r,
    r.start + 0.00000001 >= r.end,
    (r.start + r.end) / 2.0
);

// the minimum index in range s.t. prop holds
pub fn binsearch<X: Integer + Copy>(range: std::ops::Range<X>, prop: &dyn Fn(X) -> bool) -> X {
    if prop(range.start) {
        range.start
    } else {
        let mut left = range.start;
        let mut right = range.end;
        while !X::close(left..right) {
            let mid = X::middle(left..right);
            if prop(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}

#[cfg(test)]
mod test_binary_search {
    use crate::algorithm::binary_search::*;

    #[test]
    fn search_bound() {
        let v: Vec<usize> = (0..100).collect();
        assert_eq!(binsearch(0..100, &|i| v[i] > 50), 51);
        assert_eq!(binsearch(0..100, &|i| v[i] >= 0), 0);
        assert_eq!(binsearch(0..100, &|i| v[i] > 100), 100);
    }
}
