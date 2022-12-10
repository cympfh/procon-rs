/// collections - multiset
#[derive(Debug, Clone)]
pub struct MultiSet<K>
where
    K: Eq + std::hash::Hash,
{
    data: std::collections::HashMap<K, usize>,
    size: usize,
}
impl<K: Eq + std::hash::Hash> MultiSet<K> {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
            size: 0,
        }
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<K, usize> {
        self.data.keys()
    }
    pub fn insert(&mut self, item: K) {
        match self.data.get(&item) {
            None => self.data.insert(item, 1),
            Some(&c) => self.data.insert(item, c + 1),
        };
        self.size += 1;
    }
    pub fn remove(&mut self, item: K) {
        match self.data.get(&item) {
            None | Some(0) => panic!("multiset remove"),
            Some(1) => self.data.remove(&item),
            Some(&c) => self.data.insert(item, c - 1),
        };
        self.size -= 1;
    }
    pub fn uniq_size(&self) -> usize {
        self.data.len()
    }
    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod test_multiset {
    use crate::collections::multiset::*;

    #[test]
    fn test_multiset() {
        #[derive(PartialEq, Eq, Hash)]
        enum E {
            A,
            B,
        }
        use E::*;
        let mut c = MultiSet::new();
        c.insert(A);
        assert!(c.size == 1);
        assert!(c.uniq_size() == 1);
        c.insert(A);
        assert!(c.size == 2);
        assert!(c.uniq_size() == 1);
        c.insert(B);
        assert!(c.size == 3);
        assert!(c.uniq_size() == 2);
        c.remove(A);
        assert!(c.size == 2);
        assert!(c.uniq_size() == 2);
        c.remove(A);
        assert!(c.size == 1);
        assert!(c.uniq_size() == 1);
    }
}
