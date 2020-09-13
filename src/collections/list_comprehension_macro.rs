#[macro_export]
macro_rules! list {
    ($($t:tt)*) => {{ let mut r = vec![]; list_inner!(r, $($t)*); r }}
}
#[macro_export]
macro_rules! list_inner {
    ($r:expr) => {};
    ($r:expr, $e:expr) => { $r.push($e) };
    ($r:expr, $e:expr;) => { $r.push($e) };
    ($r:expr, $e:expr; for $k:ident in $range:expr) => { for $k in $range { list_inner!($r, $e) } };
    ($r:expr, $e:expr; if $cond:expr) => { if $cond { list_inner!($r, $e) } };
    ($r:expr, $e:expr; for $k:ident in $range:expr ; $($t:tt)*) => { for $k in $range { list_inner!($r, $e ; $($t)*); } };
    ($r:expr, $e:expr; if $cond:expr ; $($t:tt)*) => { if $cond { list_inner!($r, $e ; $($t)*); } };
}

#[cfg(test)]
mod test_list_macro {
    #[test]
    fn it_works() {
        assert_eq!(
            list! { x ; for x in 0..10 },
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            list! { x ; for x in 0..10; if x % 2 == 0 },
            vec![0, 2, 4, 6, 8]
        );
        assert_eq!(
            list! {
                (x, y);
                for x in 0..10; if x % 2 == 0;
                for y in 0..10; if x + y == 4;
            },
            vec![(0, 4), (2, 2), (4, 0)]
        );
    }
}
