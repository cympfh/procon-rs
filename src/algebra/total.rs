/// Algebra - Totalize PartialOrd
/// Thanks to: https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Total<T>(T);
impl<T> Total<T> {
    fn unwrap(self) -> T {
        self.0
    }
}
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, rhs: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&rhs.0).unwrap()
    }
}

#[cfg(test)]
mod test_total {
    use crate::algebra::total::*;
    #[test]
    fn it_works() {
        assert_eq!(Total(0.0).unwrap(), 0.0);
        let mut v: Vec<Total<f32>> = vec![Total(2.0), Total(1.0), Total(0.0)];
        v.sort();
        assert_eq!(v, vec![Total(0.0), Total(1.0), Total(2.0)]);
    }
}
