/// Hash - RollingHash
use crate::algebra::modint::*;
use crate::mint; // IGNORE

#[derive(Debug, Clone)]
pub struct RollingHash {
    data: ModInt,
    base: ModInt,
    baseinv: ModInt,
    length: usize,
    pow: ModInt,
}
impl RollingHash {
    pub fn new() -> Self {
        let data = mint!(0);
        let base = mint!(37);
        let baseinv = base.inv();
        let pow = mint!(1);
        Self {
            data,
            base,
            baseinv,
            length: 0,
            pow,
        }
    }
    pub fn unwrap(&self) -> i64 {
        self.data.0
    }
    pub fn push_back(&mut self, x: i64) {
        self.length += 1;
        self.pow = self.pow * self.base;
        self.data = self.data * self.base + mint!(x);
    }
    pub fn push_front(&mut self, x: i64) {
        self.data = self.data + self.pow * mint!(x);
        self.length += 1;
        self.pow = self.pow * self.base;
    }
    pub fn pop_back(&mut self, x: i64) {
        self.length -= 1;
        self.pow = self.pow * self.baseinv;
        self.data = (self.data - mint!(x)) * self.baseinv;
    }
    pub fn pop_front(&mut self, x: i64) {
        self.length -= 1;
        self.pow = self.pow * self.baseinv;
        self.data = self.data - self.pow * mint!(x);
    }
    pub fn concat(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.length = self.length + other.length;
        r.data = self.data * other.pow + other.data;
        r.pow = self.pow * other.pow;
        r
    }
}
impl std::cmp::PartialEq for RollingHash {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.data == other.data
    }
}
impl std::cmp::Eq for RollingHash {}
