/// Algebra - Matrix
use crate::algebra::ring::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K> {
    data: Vec<Vec<K>>,
}
#[macro_export]
macro_rules! mat {
    ( $( $( $x:expr ),* );* ) => ( Matrix::new( vec![ $( vec![ $( $x ),* ] ),* ] ) )
}
impl<K: Copy + Ring> Matrix<K> {
    pub fn new(data: Vec<Vec<K>>) -> Self {
        Matrix { data: data }
    }
    pub fn size(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
    pub fn eye(n: usize) -> Matrix<K> {
        let mut e = vec![vec![K::zero(); n]; n];
        for i in 0..n {
            e[i][i] = K::one();
        }
        Matrix::new(e)
    }
    pub fn zero(h: usize, w: usize) -> Matrix<K> {
        Matrix::new(vec![vec![K::zero(); w]; h])
    }
    pub fn pow(&self, n: usize) -> Matrix<K> {
        if n == 0 {
            Matrix::eye(self.data.len())
        } else if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            let m = self * self;
            m.pow(n / 2)
        } else {
            let m = self * self;
            &m.pow(n / 2) * self
        }
    }
    pub fn sum(&self) -> K {
        self.data
            .iter()
            .map(|row| row.iter().map(|&x| x).sum::<K>())
            .sum()
    }
    pub fn map<F>(&self, f: F) -> Matrix<K>
    where
        F: Fn(&K) -> K,
    {
        let data = self
            .data
            .iter()
            .map(|row| row.iter().map(&f).collect())
            .collect();
        Matrix::new(data)
    }
    /// O(n!)
    /// self should be square matrix.
    pub fn det(&self) -> K {
        let (n, m) = self.size();
        assert!(n == m);
        if n == 1 {
            return self.data[0][0];
        }
        let mut b = Matrix::<K>::zero(n - 1, m - 1);
        let mut d = K::zero();
        for i in 0..n {
            for bi in 0..n - 1 {
                for bj in 0..n - 1 {
                    let ai = if bi < i { bi } else { bi + 1 };
                    b.data[bi][bj] = self.data[ai][1 + bj];
                }
            }
            d = d + (if i % 2 == 0 { K::one() } else { -K::one() }) * self.data[i][0] * b.det();
        }
        d
    }
}
impl<K: Copy + Ring> Matrix<K>
where
    K: std::ops::RemAssign + std::ops::Rem<Output = K>,
{
    pub fn powmod(&self, n: u64, modulo: K) -> Matrix<K> {
        if n == 0 {
            Matrix::eye(self.data.len())
        } else if n == 1 {
            self.map(|&x| x % modulo)
        } else {
            let mut m = (self * self).map(|&x| x % modulo);
            m = m.powmod(n / 2, modulo);
            if n % 2 == 1 {
                m = (&m * self).map(|&x| x % modulo);
            }
            m
        }
    }
}
// -M
impl<K: Copy + Ring> std::ops::Neg for &Matrix<K> {
    type Output = Matrix<K>;
    fn neg(self) -> Matrix<K> {
        self.map(|&x| -x)
    }
}
// M + N
impl<K: Copy + Ring> std::ops::Add<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: &Matrix<K>) -> Matrix<K> {
        let (h, w) = self.size();
        let data = (0..h)
            .map(|i| (0..w).map(|j| self.data[i][j] + other.data[i][j]).collect())
            .collect();
        Matrix::new(data)
    }
}
// M - N
impl<K: Copy + Ring> std::ops::Sub<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, other: &Matrix<K>) -> Matrix<K> {
        self + &-other
    }
}
// M * N
impl<K: Copy + Ring> std::ops::Mul<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, other: &Matrix<K>) -> Matrix<K> {
        let (h, w) = self.size();
        let (_, v) = other.size();
        let data = (0..h)
            .map(|i| {
                (0..v)
                    .map(|k| (0..w).map(|j| self.data[i][j] * other.data[j][k]).sum())
                    .collect()
            })
            .collect();
        Matrix::new(data)
    }
}
// M * k
impl<K: Copy + Ring> std::ops::Mul<K> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, k: K) -> Matrix<K> {
        self.map(|&x| x * k)
    }
}

#[cfg(test)]
mod test_matrix {
    #[test]
    fn it_works() {
        use crate::algebra::matrix::*;
        {
            let m = mat![0, -1; 1, 0];
            assert_eq!(&m * &m, mat![-1, 0; 0, -1]);
            assert_eq!(m.pow(2), mat![-1, 0; 0, -1]);
            assert_eq!(m.powmod(2, 10), mat![-1, 0; 0, -1]);
            assert_eq!(-&m, mat![0, 1; -1, 0]);
            assert_eq!(&m + &Matrix::zero(2, 2), m);
            assert_eq!(&m + &Matrix::eye(2), mat![1, -1; 1, 1]);
        }
    }
}
