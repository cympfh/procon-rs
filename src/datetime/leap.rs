/// Calendar - Test Leap Year (うるう年)
pub fn is_leap(y: usize) -> bool {
    y % 4 == 0 && y % 100 != 0 || y % 400 == 0
}
#[cfg(test)]
mod test_leap {
    #[test]
    fn it_works() {
        use crate::datetime::leap::*;
        assert!(!is_leap(1900));
        assert!(!is_leap(1999));
        assert!(is_leap(2000));
        assert!(!is_leap(2003));
        assert!(is_leap(2004));
        assert!(!is_leap(2019));
        assert!(is_leap(2020));
        assert!(!is_leap(2100));
    }
}
