/// Xorshift128 PRNG matching JavaScript's bitwise semantics.
///
/// JS bitwise operators (`<<`, `>>`, `^`) coerce to signed 32-bit (ToInt32),
/// while `>>> 0` coerces to unsigned 32-bit (ToUint32). We use `i32` for state
/// and cast to `u32` for the final `>>> 0` and division step.
pub struct Xorshift128 {
    s: [i32; 4],
}

impl Xorshift128 {
    /// Seed the PRNG from a string, matching JS `seedRand()`.
    ///
    /// The seeding operation `s[i%4] = (s[i%4] << 5) - s[i%4] + charCode`
    /// is equivalent to `s[i%4] * 31 + charCode` using i32 wrapping arithmetic.
    pub fn new(seed: &str) -> Self {
        let mut s = [0i32; 4];
        for (i, ch) in seed.chars().enumerate() {
            let code = ch as i32;
            let idx = i % 4;
            s[idx] = s[idx]
                .wrapping_shl(5)
                .wrapping_sub(s[idx])
                .wrapping_add(code);
        }
        Self { s }
    }

    /// Step the PRNG and return a float in [0, 1), matching JS `rand()`.
    pub fn next_f64(&mut self) -> f64 {
        // t = s[0] ^ (s[0] << 11)
        let t = self.s[0] ^ self.s[0].wrapping_shl(11);

        // Rotate: s[0]=s[1], s[1]=s[2], s[2]=s[3]
        self.s[0] = self.s[1];
        self.s[1] = self.s[2];
        self.s[2] = self.s[3];

        // s[3] = (s[3] ^ (s[3] >> 19) ^ t ^ (t >> 8)) >>> 0
        // The >> in JS is a signed right shift (ToInt32), and >>> 0 converts to u32.
        let result = self.s[3] ^ (self.s[3] >> 19) ^ t ^ (t >> 8);
        let unsigned = result as u32;
        self.s[3] = unsigned as i32;

        // return s[3] / ((1 << 31) >>> 0)
        // (1 << 31) >>> 0 === 2147483648
        unsigned as f64 / 2_147_483_648.0
    }

    /// Get the current state, for test vector validation.
    pub fn seeded_state(&self) -> [i32; 4] {
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_seed() {
        let mut rng = Xorshift128::new("");
        assert_eq!(rng.s, [0, 0, 0, 0]);
        // With all-zero state, output should be 0.0
        assert_eq!(rng.next_f64(), 0.0);
    }

    #[test]
    fn known_seed() {
        let rng = Xorshift128::new("abcd1234");
        assert_eq!(rng.s, [3056, 3088, 3120, 3152]);
    }

    #[test]
    fn deterministic() {
        let mut rng1 = Xorshift128::new("test");
        let mut rng2 = Xorshift128::new("test");
        for _ in 0..100 {
            assert_eq!(rng1.next_f64(), rng2.next_f64());
        }
    }
}
