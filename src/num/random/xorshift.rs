/// Random Number - Xor-Shift Algorithm
use crate::num::random::fromu64::*;

pub struct XorShift(u64);
impl XorShift {
    pub fn new() -> Self {
        XorShift(88_172_645_463_325_252)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.0 = x;
        x
    }
    pub fn gen<T: FromU64>(&mut self) -> T {
        T::coerce(self.next())
    }
}

#[cfg(test)]
mod test_xorshift {
    use crate::num::random::xorshift::*;

    #[test]
    fn it_works() {
        let mut rand = XorShift::new();
        let _: bool = rand.gen();
        let _: usize = rand.gen();
        let _: i32 = rand.gen();
        let _: u32 = rand.gen();
        let _: f64 = rand.gen();
    }
}
