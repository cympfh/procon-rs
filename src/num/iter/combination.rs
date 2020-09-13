/// Number - Iterator - Combination (Binom[n;m])
pub struct Combination {
    n: usize,
    m: usize,
    ar: Vec<usize>,
}
impl Combination {
    pub fn new(n: usize, m: usize) -> Combination {
        let mut ar = vec![0; m];
        for i in 0..m {
            ar[i] = m - i - 1
        }
        Combination { n: n, m: m, ar: ar }
    }
}
impl Iterator for Combination {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.m == 0 {
            if self.n == 0 {
                return None;
            } else {
                self.n = 0;
                return Some(vec![]);
            }
        }
        if self.ar[self.m - 1] > self.n - self.m {
            return None;
        }
        let r = self.ar.clone();
        self.ar[0] += 1;
        let mut c = 0;
        for i in 0..self.m - 1 {
            if self.ar[i] >= self.n - i {
                self.ar[i + 1] += 1;
                c = i + 1;
            } else {
                break;
            }
        }
        for i in (0..c).rev() {
            self.ar[i] = self.ar[i + 1] + 1;
        }
        return Some(r);
    }
}

#[cfg(test)]
mod test_combination_perm {
    use crate::num::iter::combination::*;

    #[test]
    fn it_works() {
        let mut perm = Combination::new(3, 2);
        assert_eq!(perm.next(), Some(vec![1, 0]));
        assert_eq!(perm.next(), Some(vec![2, 0]));
        assert_eq!(perm.next(), Some(vec![2, 1]));
        assert_eq!(perm.next(), None);
    }
}
