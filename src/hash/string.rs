/// Hash - String Hash (by ZobristHash & RollingHash)
use crate::hash::rolling::*;
use crate::hash::zobrist::*;
use crate::num::random::xorshift::*;

#[derive(Debug, Clone)]
pub struct StringHash {
    length: usize,
    sethash: ZobristHash,
    rolhash: RollingHash,
    table: Vec<i64>, // char -> int
}
impl StringHash {
    pub fn new() -> Self {
        let length = 0;
        let sethash = ZobristHash::new();
        let rolhash = RollingHash::new();
        let mut rng = XorShift::new();
        let table: Vec<i64> = (0..300)
            .map(|_| {
                let max = 1_000_000_007;
                rng.gen::<i64>() % max
            })
            .collect();
        Self {
            length,
            sethash,
            rolhash,
            table,
        }
    }
    pub fn push_front(&mut self, c: char) {
        let x = c as usize;
        self.sethash.add(self.table[x]);
        self.rolhash.push_front(self.table[x]);
        self.length += 1;
    }
    pub fn push_back(&mut self, c: char) {
        let x = c as usize;
        self.sethash.add(self.table[x]);
        self.rolhash.push_back(self.table[x]);
        self.length += 1;
    }
    pub fn pop_front(&mut self, c: char) {
        let x = c as usize;
        self.sethash.remove(self.table[x]);
        self.rolhash.pop_front(self.table[x]);
        self.length -= 1;
    }
    pub fn pop_back(&mut self, c: char) {
        let x = c as usize;
        self.length -= 1;
        self.sethash.remove(self.table[x]);
        self.rolhash.pop_back(self.table[x]);
    }
    pub fn concat(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.length = self.length + other.length;
        r.sethash = self.sethash.concat(&other.sethash);
        r.rolhash = self.rolhash.concat(&other.rolhash);
        r
    }
}
impl std::cmp::PartialEq for StringHash {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.sethash == other.sethash
            && self.rolhash == other.rolhash
    }
}
impl std::cmp::Eq for StringHash {}

#[cfg(test)]
mod test_hash_string {
    use crate::hash::string::*;
    #[test]
    fn test_hash_string() {
        // '' == ''
        let mut x = StringHash::new();
        let mut y = StringHash::new();
        assert_eq!(x, y);
        // 'abc' != 'bc'
        x.push_back('a');
        x.push_back('b');
        x.push_back('c');
        y.push_back('c');
        y.push_front('b');
        assert_ne!(x, y);
        // 'abc' == 'abc'
        y.push_front('a');
        assert_eq!(x, y);
        // '(abc' != 'abc)'
        x.push_front('(');
        y.push_back(')');
        assert_ne!(x, y);
        // '(abc)' == '(abc)'
        x.push_back(')');
        y.push_front('(');
        assert_eq!(x, y);
    }
}
