/// Misc - Neighbor
pub mod neighbor {
    pub struct Grid4(pub usize, pub usize);
    impl Grid4 {
        pub fn iter(&self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s + t) % 2 == 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct Grid8(pub usize, pub usize);
    impl Grid8 {
        pub fn iter<'a>(&'a self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s * t) != 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct VecIter<T>(Vec<T>);
    impl<T: Copy> Iterator for VecIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }
}

#[cfg(test)]
mod test_list_macro {
    #[test]
    fn it_works() {
        use crate::misc::neighbor::*;
        {
            let mut neighs = neighbor::Grid4(3, 4).iter(0, 0);
            assert_eq!(neighs.next(), Some((1, 0)));
            assert_eq!(neighs.next(), Some((0, 1)));
            assert_eq!(neighs.next(), None);
        }
        {
            let mut neighs = neighbor::Grid8(3, 4).iter(0, 0);
            assert_eq!(neighs.next(), Some((1, 1)));
            assert_eq!(neighs.next(), Some((1, 0)));
            assert_eq!(neighs.next(), Some((0, 1)));
            assert_eq!(neighs.next(), None);
        }
        {
            let mut neighs = neighbor::Grid4(10, 10).iter(3, 3);
            assert_eq!(neighs.next(), Some((4, 3)));
            assert_eq!(neighs.next(), Some((3, 4)));
            assert_eq!(neighs.next(), Some((3, 2)));
            assert_eq!(neighs.next(), Some((2, 3)));
            assert_eq!(neighs.next(), None);
        }
    }
}
