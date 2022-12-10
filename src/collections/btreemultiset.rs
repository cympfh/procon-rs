/// collections - BTree MultiSet
#[derive(Debug, Clone)]
pub struct BTreeMultiSet<T> {
    data: std::collections::BTreeMap<T, usize>,
    size: usize,
}
impl<T: Sized + Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        let data = std::collections::BTreeMap::new();
        let size = 0;
        Self { data, size }
    }
    pub fn insert(&mut self, item: T) {
        self.data.entry(item).and_modify(|e| *e += 1).or_insert(1);
        self.size += 1;
    }
    pub fn get(&self, item: &T) -> Option<usize> {
        self.data.get(item).cloned()
    }
    pub fn remove(&mut self, item: T) {
        if let Some(&c) = self.data.get(&item) {
            if c <= 1 {
                self.data.remove(&item);
            } else {
                self.data.entry(item).and_modify(|e| *e -= 1);
            }
            self.size -= 1;
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn range<R: std::ops::RangeBounds<T>>(
        &self,
        range: R,
    ) -> std::collections::btree_map::Range<T, usize> {
        self.data.range(range)
    }
    pub fn min<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.data.range(range).next().map(|(t, _)| t)
    }
    pub fn max<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.data.range(range).next_back().map(|(t, _)| t)
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

    #[test]
    fn test_multiset_len() {
        let mut c = BTreeMultiSet::new();
        c.insert(123);
        c.insert(123);
        assert_eq!(c.len(), 2);
        c.remove(123);
        assert_eq!(c.len(), 1);
    }
}
