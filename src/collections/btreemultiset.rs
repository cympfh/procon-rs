/// collections - BTree MultiSet
#[derive(Debug, Clone)]
pub struct BTreeMultiSet<T>(std::collections::BTreeMap<T, usize>);
impl<T: Sized + Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }
    pub fn insert(&mut self, item: T) {
        self.0.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }
    pub fn get(&self, item: &T) -> Option<usize> {
        self.0.get(item).cloned()
    }
    pub fn remove(&mut self, item: T) {
        if let Some(&c) = self.0.get(&item) {
            if c <= 1 {
                self.0.remove(&item);
            } else {
                self.0.entry(item).and_modify(|e| *e -= 1);
            }
        }
    }
    pub fn range<R: std::ops::RangeBounds<T>>(
        &self,
        range: R,
    ) -> std::collections::btree_map::Range<T, usize> {
        self.0.range(range)
    }
    pub fn min<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.0.range(range).next().map(|(t, _)| t)
    }
    pub fn max<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.0.range(range).next_back().map(|(t, _)| t)
    }
}

#[cfg(test)]
mod test_btree_multiset {
    use crate::collections::btreemultiset::*;

    #[test]
    fn test_multiset() {
        let mut c = BTreeMultiSet::new();
        assert_eq!(None, c.get(&"hoge"));
        c.insert("hoge");
        assert_eq!(Some(1), c.get(&"hoge"));
        c.insert("hoge");
        assert_eq!(Some(2), c.get(&"hoge"));
        c.remove("hoge");
        assert_eq!(Some(1), c.get(&"hoge"));
        c.remove("hoge");
        assert_eq!(None, c.get(&"hoge"));
        c.remove("hoge");
        assert_eq!(None, c.get(&"hoge"));
    }
}
