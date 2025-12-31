/// Random Number - Permuted Congruential Generators (PCG64)
///
/// This is an implementation of PCG64 (Lcg128Xsl64):
/// - 128-bit internal state
/// - 64-bit output
/// - Uses XSL-RR (XorShift Low, Random Rotation) output function
use crate::num::random::fromu64::*;

pub struct PCG {
    state: u128,
    increment: u128,
}

impl PCG {
    /// Default multiplier for 128-bit LCG
    const MULTIPLIER: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;

    /// Create a new PCG with default stream
    pub fn new(seed: u64) -> Self {
        Self::with_stream(seed, 0)
    }

    /// Create a new PCG with specified seed and stream
    /// The stream parameter allows creating independent random number sequences
    pub fn with_stream(seed: u64, stream: u64) -> Self {
        // Increment must be odd, so we use: (stream << 1) | 1
        let increment = ((stream as u128) << 1) | 1;

        let mut pcg = Self {
            state: 0,
            increment,
        };

        // Initialize the state properly
        pcg.state = pcg.state.wrapping_add(seed as u128);
        pcg.step();
        pcg.state = pcg.state.wrapping_add(seed as u128);
        pcg.step();

        pcg
    }

    /// Advance the internal state (LCG step)
    #[inline]
    fn step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(self.increment);
    }

    /// Generate next random u64 using XSL-RR output function
    fn next(&mut self) -> u64 {
        self.step();

        // XSL-RR output function
        // Extract rotation amount from top bits (bits 122-127)
        let rot = (self.state >> 122) as u32;

        // XOR high and low 64 bits
        let xored = ((self.state >> 64) as u64) ^ (self.state as u64);

        // Rotate right by rot
        xored.rotate_right(rot)
    }

    /// Generate a value of type T from the random u64
    pub fn gen<T: FromU64>(&mut self) -> T {
        T::coerce(self.next())
    }
}

#[cfg(test)]
mod test_pcg {
    use crate::num::random::pcg::*;

    #[test]
    fn it_works() {
        let mut rand = PCG::new(42);
        let _: bool = rand.gen();
        let _: usize = rand.gen();
        let _: i32 = rand.gen();
        let _: u32 = rand.gen();
        let _: f64 = rand.gen();
    }

    #[test]
    fn generates_different_values() {
        let mut rand = PCG::new(12345);
        let a: u64 = rand.gen();
        let b: u64 = rand.gen();
        let c: u64 = rand.gen();

        // Should generate different values
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn same_seed_same_sequence() {
        let mut rand1 = PCG::new(999);
        let mut rand2 = PCG::new(999);

        for _ in 0..10 {
            let a: u64 = rand1.gen();
            let b: u64 = rand2.gen();
            assert_eq!(a, b, "Same seed should produce same sequence");
        }
    }

    #[test]
    fn different_streams() {
        let mut rand1 = PCG::with_stream(42, 0);
        let mut rand2 = PCG::with_stream(42, 1);

        let a: u64 = rand1.gen();
        let b: u64 = rand2.gen();

        // Different streams should produce different values
        assert_ne!(a, b);
    }
}
