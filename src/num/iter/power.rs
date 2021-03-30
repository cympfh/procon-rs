/// Number - Iterator - Power Permutation (pow(n, m))
pub struct PowerPermutation {
    n: usize,
    m: usize,
    ar: Vec<usize>,
    done: bool,
}
impl PowerPermutation {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            ar: vec![0; m],
            done: false,
        }
    }
}
impl Iterator for PowerPermutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.m == 0 {
            self.done = true;
            return Some(self.ar.clone());
        }
        if self.ar[self.m - 1] >= self.n {
            return None;
        }
        let r = self.ar.clone();
        self.ar[0] += 1;
        for i in 0..self.m - 1 {
            if self.ar[i] == self.n {
                self.ar[i] = 0;
                self.ar[i + 1] += 1;
            } else {
                break;
            }
        }
        Some(r)
    }
}

#[cfg(test)]
mod test_power_perm {
    use crate::num::iter::power::*;

    #[test]
    fn it_works() {
        let mut perm = PowerPermutation::new(3, 2);
        assert_eq!(perm.next(), Some(vec![0, 0]));
        assert_eq!(perm.next(), Some(vec![1, 0]));
        assert_eq!(perm.next(), Some(vec![2, 0]));
        assert_eq!(perm.next(), Some(vec![0, 1]));
        assert_eq!(perm.next(), Some(vec![1, 1]));
        assert_eq!(perm.next(), Some(vec![2, 1]));
        assert_eq!(perm.next(), Some(vec![0, 2]));
        assert_eq!(perm.next(), Some(vec![1, 2]));
        assert_eq!(perm.next(), Some(vec![2, 2]));
        assert_eq!(perm.next(), None);
    }

    #[test]
    fn test_small() {
        let mut perm = PowerPermutation::new(2, 1);
        assert_eq!(perm.next(), Some(vec![0]));
        assert_eq!(perm.next(), Some(vec![1]));
        assert_eq!(perm.next(), None);
    }

    #[test]
    fn test_take_zero() {
        let mut perm = PowerPermutation::new(2, 0);
        assert_eq!(perm.next(), Some(vec![]));
        assert_eq!(perm.next(), None);
    }

    #[test]
    fn test_take_zero_from_zero() {
        let mut perm = PowerPermutation::new(0, 0);
        assert_eq!(perm.next(), Some(vec![]));
        assert_eq!(perm.next(), None);
    }

    #[test]
    fn test_from_zero() {
        let mut perm = PowerPermutation::new(0, 2);
        assert_eq!(perm.next(), None);
    }
}
