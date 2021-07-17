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
    pub fn add(&mut self, item: K) {
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
}

#[cfg(test)]
mod test_multiset {
    #[test]
    fn test_multiset() {
        #[derive(PartialEq, Eq)]
        enum E {
            A,
            B,
            C,
        }
        use E::*;
        let c = MultiSet::new();
        c.add(A);
        assert(c.size == 1);
        assert(c.uniq_size() == 1);
        c.add(A);
        assert(c.size == 2);
        assert(c.uniq_size() == 1);
        c.add(B);
        assert(c.size == 3);
        assert(c.uniq_size() == 2);
        c.remove(A);
        assert(c.size == 2);
        assert(c.uniq_size() == 2);
        c.remove(A);
        assert(c.size == 1);
        assert(c.uniq_size() == 1);
    }
}
