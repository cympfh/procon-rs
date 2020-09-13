/// collections - defaultdict
#[derive(Debug, Clone)]
pub struct DefaultDict<K, V> {
    data: std::collections::HashMap<K, V>,
    default: V,
}
impl<K: Eq + std::hash::Hash, V> DefaultDict<K, V> {
    pub fn new(default: V) -> DefaultDict<K, V> {
        DefaultDict {
            data: std::collections::HashMap::new(),
            default,
        }
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<K, V> {
        self.data.keys()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl<K: Eq + std::hash::Hash, V> std::ops::Index<K> for DefaultDict<K, V> {
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        if let Some(val) = self.data.get(&key) {
            val
        } else {
            &self.default
        }
    }
}
impl<K: Eq + std::hash::Hash + Clone, V: Clone> std::ops::IndexMut<K> for DefaultDict<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        let val = self.default.clone();
        self.data.entry(key.clone()).or_insert(val);
        self.data.get_mut(&key).unwrap()
    }
}

#[cfg(test)]
mod test_defaultdict {
    #[test]
    fn it_works() {
        use crate::collections::defaultdict::*;
        {
            let mut m = DefaultDict::new(0);
            m['a'] += 1;
            m['a'] += 1;
            assert_eq!(m['a'], 2);
            assert_eq!(m['b'], 0);
            m['b'] = 10;
            assert_eq!(m['b'], 10);
            m['b'] -= 1;
            assert_eq!(m['b'], 9);
        }
        {
            let mut m = DefaultDict::new(vec![]);
            m[0].push(2);
            assert_eq!(m[0], vec![2]);
        }
    }
}
