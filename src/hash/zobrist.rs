/// Hash - Zobrist Hash
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ZobristHash(i64);
impl ZobristHash {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn add(&mut self, x: i64) {
        self.0 ^= x;
    }
    pub fn remove(&mut self, x: i64) {
        self.0 ^= x;
    }
    pub fn concat(&self, other: &ZobristHash) -> Self {
        Self(self.0 ^ other.0)
    }
}
