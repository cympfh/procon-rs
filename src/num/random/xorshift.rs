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
    pub fn shuffle<T>(&mut self, xs: &mut Vec<T>) {
        for i in (0..xs.len()).rev() {
            let j = self.gen::<usize>() % (i + 1);
            xs.swap(i, j);
        }
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

    #[test]
    fn test_shuffle() {
        let mut rand = XorShift::new();
        let mut xs = vec![3, 2, 1];
        rand.shuffle(&mut xs);
        xs.sort();
        assert_eq!(xs, vec![1, 2, 3]);
    }
}
