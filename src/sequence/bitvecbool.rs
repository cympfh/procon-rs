/// Bitmask を用いた Vec<Bool> のシミュレート実装
struct BitVecBool {
    data: u128,
}
impl BitVecBool {
    fn new() -> Self {
        Self { data: 0 }
    }
    fn from(data: u128) -> Self {
        Self { data }
    }
    /// self[i]
    fn get(&self, i: usize) -> bool {
        (self.data >> i) & 1 == 1
    }
    /// self[i] = value;
    fn set(&mut self, i: usize, value: bool) {
        if value {
            self.data |= 1 << i;
        } else {
            self.data &= !(1 << i);
        }
    }
    fn count_true(&self) -> usize {
        self.data.count_ones() as usize
    }
    /// iter on value
    fn iter(&self, value: bool) -> impl Iterator<Item = usize> + '_ {
        (0..128).filter(move |&i| self.get(i) == value)
    }
}
