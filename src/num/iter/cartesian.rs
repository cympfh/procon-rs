/// Number - Iterator - Cartesian Product (zip) Permutation (n x m)
pub struct CartesianPermutation {
    dim: usize,
    size: Vec<usize>,
    data: Vec<usize>,
    done: bool,
}
#[macro_export]
macro_rules! zip {
    ($($xs:expr),* $(,)?) => { CartesianPermutation::new(vec![$($xs),*]) }
}
impl CartesianPermutation {
    pub fn new(size: Vec<usize>) -> Self {
        let dim = size.len();
        let done = size.iter().any(|&s| s == 0);
        Self {
            dim,
            size,
            data: vec![0; dim],
            done,
        }
    }
}
impl Iterator for CartesianPermutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        let ret = self.data.clone();
        self.data[0] += 1;
        for i in 0..self.dim {
            if self.data[i] >= self.size[i] {
                self.data[i] -= self.size[i];
                if i < self.dim - 1 {
                    self.data[i + 1] += 1;
                } else {
                    self.done = true;
                }
            }
        }
        Some(ret)
    }
}

#[cfg(test)]
mod test_cartesian {
    use crate::num::iter::cartesian::*;

    #[test]
    fn it_works() {
        let mut prod = CartesianPermutation::new(vec![3]);
        assert_eq!(prod.next(), Some(vec![0]));
        assert_eq!(prod.next(), Some(vec![1]));
        assert_eq!(prod.next(), Some(vec![2]));
        assert_eq!(prod.next(), None);

        let mut prod = CartesianPermutation::new(vec![3, 2, 1]);
        assert_eq!(prod.next(), Some(vec![0, 0, 0]));
        assert_eq!(prod.next(), Some(vec![1, 0, 0]));
        assert_eq!(prod.next(), Some(vec![2, 0, 0]));
        assert_eq!(prod.next(), Some(vec![0, 1, 0]));
        assert_eq!(prod.next(), Some(vec![1, 1, 0]));
        assert_eq!(prod.next(), Some(vec![2, 1, 0]));
        assert_eq!(prod.next(), None);

        let mut prod = CartesianPermutation::new(vec![3, 0, 2, 1]);
        assert_eq!(prod.next(), None);

        let mut prod = CartesianPermutation::new(vec![2, 2]);
        assert_eq!(prod.next(), Some(vec![0, 0]));
        assert_eq!(prod.next(), Some(vec![1, 0]));
        assert_eq!(prod.next(), Some(vec![0, 1]));
        assert_eq!(prod.next(), Some(vec![1, 1]));
        assert_eq!(prod.next(), None);
    }
}
