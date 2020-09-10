/// Set - Union-Find
#[derive(Debug, Clone)]
pub struct UnionFind {
    data: Vec<UF>,
}

#[derive(Debug, Clone)]
enum UF {
    Root(usize),
    Child(usize),
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![UF::Root(1); n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        match self.data[x] {
            UF::Root(_) => x,
            UF::Child(parent) => {
                let root = self.root(parent);
                self.data[x] = UF::Child(root);
                root
            }
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        match self.data[r] {
            UF::Root(size) => size,
            UF::Child(_) => 0,
        }
    }
    pub fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            let size_x = self.size(root_x);
            let size_y = self.size(root_y);
            let (i, j) = if size_x > size_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.data[i] = UF::Root(size_x + size_y);
            self.data[j] = UF::Child(i);
        }
    }
}

#[cfg(test)]
mod test_union_find {
    use crate::set::union_find::*;

    #[test]
    fn it_works() {
        let mut uf = UnionFind::new(10);
        for i in 0..10 {
            assert!(uf.is_same(i, i));
            for j in 0..i {
                assert!(!uf.is_same(i, j));
            }
        }

        uf.merge(0, 1);
        assert!(uf.is_same(0, 1));

        uf.merge(2, 3);
        assert!(uf.is_same(2, 3));
        assert!(!uf.is_same(0, 3));

        uf.merge(0, 3);
        for i in 0..4 {
            for j in 0..4 {
                assert!(uf.is_same(i, j));
            }
        }
    }
}
