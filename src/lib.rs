/// PCG32 random number generator.
#[derive(Copy, Clone)]
pub struct Pcg32 {
    state: u64,
    stream: u64,
}

impl Pcg32 {
    pub fn with_seed_and_sequence(initial_state: u64, sequence: u64) -> Self {
        let mut g = Self {
            state: initial_state,
            stream: (sequence << 1) | 1,
        };
        g.next_u32();
        g.state += initial_state;
        g.next_u32();
        g
    }

    /// Create generator with random seed
    pub fn with_seed(initial_state: u64) -> Self {
        Self::with_seed_and_sequence(initial_state, 1)
    }

    /// Request next u32 from the generator.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        const PCG32_MULTIPLIER: u64 = 0x5851f42d4c957f2d;

        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(PCG32_MULTIPLIER)
            .wrapping_add(self.stream);
        let xor_shifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xor_shifted >> rot) | (xor_shifted << ((!rot).wrapping_add(1) & 31))
    }

    /// Request next f32 from the generator.
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        let u = (self.next_u32() >> 9) | 0x3f800000;
        f32::from_bits(u) - 1.0
    }

    /// Request next f64 from the generator.
    /// # Remark
    /// Since the random number used is 32 bits, only the first 32 mantissa bits will be filled,
    /// still we use more bits compared to the f32 version which only uses 23 bits.
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        // Trick from MTGP: generate an uniformly distributed double precision number in [1,2) and subtract 1.
        let u = ((self.next_u32() as u64) << 20) | 0x3ff0000000000000;
        f64::from_bits(u) - 1.0
    }
}

impl Default for Pcg32 {
    #[inline]
    fn default() -> Self {
        const PCG32_DEFAULT_STATE: u64 = 0x853c49e6748fea9b;
        const PCG32_DEFAULT_STREAM: u64 = 0xda3e39cb94b95bdb;
        Self {
            state: PCG32_DEFAULT_STATE,
            stream: PCG32_DEFAULT_STREAM,
        }
    }
}
