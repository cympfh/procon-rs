/// Algebra - Symmetric Group (対称群), transposition (互換)
/// TODO: AtCoder の Rust がバージョンアップしてくれたら const generics で書き換える
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymmetricGroup {
    n: usize,
    p: Vec<usize>,
    pinv: Vec<usize>,
}
impl SymmetricGroup {
    pub fn new(p: Vec<usize>) -> Self {
        let n = p.len();
        let mut pinv = vec![0; n];
        for i in 0..n {
            pinv[p[i]] = i;
        }
        Self { n, p, pinv }
    }
    pub fn one(n: usize) -> Self {
        let p = (0..n).collect();
        SymmetricGroup::new(p)
    }
    /// Inverse, O(N)
    pub fn inv(&self) -> Self {
        Self {
            n: self.n,
            p: self.pinv.clone(),
            pinv: self.p.clone(),
        }
    }
    /// SymmetricGroup * SymmetricGroup, O(N)
    pub fn mul(&self, other: &SymmetricGroup) -> Self {
        let p = (0..self.n).map(|i| other.p[self.p[i]]).collect();
        SymmetricGroup::new(p)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transposition {
    x: usize,
    y: usize,
}
impl Transposition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    /// Inverse, O(1)
    pub fn inv(&self) -> Self {
        self.clone()
    }
    /// Cast to SymmetricGroup
    pub fn into_n(&self, n: usize) -> SymmetricGroup {
        let mut p: Vec<usize> = (0..n).collect();
        p[self.x] = self.y;
        p[self.y] = self.x;
        SymmetricGroup::new(p)
    }
}

impl SymmetricGroup {
    /// self=SymmetricGroup * Transposition, O(1)
    pub fn swap(&mut self, trans: &Transposition) {
        let i = self.pinv[trans.x];
        let j = self.pinv[trans.y];
        self.p.swap(i, j);
        self.pinv.swap(trans.x, trans.y);
    }
    /// Transposition * self=SymmetricGroup, O(1)
    pub fn unswap(&mut self, trans: &Transposition) {
        let i = self.p[trans.x];
        let j = self.p[trans.y];
        self.p.swap(trans.x, trans.y);
        self.pinv.swap(i, j);
    }
}

#[macro_export]
macro_rules! perm {
    (sym [ $( $val:expr ),* $(,)? ]) => {
        SymmetricGroup::new(vec![ $($val),* ])
    };
    (trans [$x:expr, $y:expr $(,)?]) => {
        Transposition::new($x, $y)
    };
}

#[cfg(test)]
mod test_symmetric {
    use crate::algebra::symmetric::*;

    #[test]
    fn test_inverse() {
        let a = SymmetricGroup::new(vec![0, 3, 1, 2]);
        let ainv = a.inv();
        assert_eq!(a.mul(&ainv), SymmetricGroup::one(4));
        assert_eq!(ainv.mul(&a), SymmetricGroup::one(4));

        let a = SymmetricGroup::new(vec![0, 3, 1, 2]);
        let b = SymmetricGroup::new(vec![3, 2, 1, 0]);
        let ab = a.mul(&b);
        assert_eq!(ab.mul(&b.inv()), a);
        assert_eq!(a.inv().mul(&ab), b);
    }

    #[test]
    fn test_mul() {
        let a = SymmetricGroup::new(vec![0, 3, 1, 2]);
        let b = SymmetricGroup::new(vec![3, 2, 1, 0]);
        assert_eq!(a.mul(&b), SymmetricGroup::new(vec![3, 0, 2, 1]));
        assert_eq!(b.mul(&a), SymmetricGroup::new(vec![2, 1, 3, 0]));
    }

    #[test]
    fn test_assoc() {
        let a = SymmetricGroup::new(vec![0, 3, 1, 2]);
        let b = SymmetricGroup::new(vec![3, 2, 1, 0]);
        let c = SymmetricGroup::new(vec![2, 0, 3, 1]);
        let ab = a.mul(&b);
        let bc = b.mul(&c);
        assert_eq!(ab.mul(&c), a.mul(&bc));
    }

    #[test]
    fn test_transposition() {
        let a = Transposition::new(2, 4);
        let s = SymmetricGroup::new(vec![0, 1, 4, 3, 2, 5]);
        assert_eq!(a.into_n(6), s);
    }

    #[test]
    fn test_swap() {
        let mut a = perm!(sym [3, 2, 1, 0]);
        a.swap(&perm!(trans [2, 3]));
        assert_eq!(a, perm!(sym [2, 3, 1, 0]));
        let mut a = perm!(sym [1, 2, 3, 0]);
        a.swap(&perm!(trans [1, 3]));
        assert_eq!(a, perm!(sym [3, 2, 1, 0]));
    }

    #[test]
    fn test_unswap() {
        let mut a = perm!(sym [3, 2, 1, 0]);
        a.unswap(&perm!(trans [2, 3]));
        assert_eq!(a, perm!(sym [3, 2, 0, 1]));
        let mut a = perm!(sym [1, 2, 3, 0]);
        a.unswap(&perm!(trans [1, 3]));
        assert_eq!(a, perm!(sym [1, 0, 3, 2]));
    }
}
