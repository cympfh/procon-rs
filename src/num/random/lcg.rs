/// Random Number - Linear Congruential Generators
use crate::num::random::fromu64::*;

pub struct LCG(u64, u64, u64, u64);
impl LCG {
    pub fn new() -> Self {
        Self(127, 48271, 0, 1 << 31 - 1) // Park & Miller
    }
    fn next(&mut self) -> u64 {
        self.0 = (self.0 * self.1 + self.2) % self.3;
        self.0
    }
    pub fn gen<T: FromU64>(&mut self) -> T {
        T::coerce(self.next())
    }
}

#[cfg(test)]
mod test_lcg {
    use crate::num::random::lcg::*;

    #[test]
    fn it_works() {
        let mut rand = LCG::new();
        let _: bool = rand.gen();
        let _: usize = rand.gen();
        let _: i32 = rand.gen();
        let _: u32 = rand.gen();
        let _: f64 = rand.gen();
    }
}
