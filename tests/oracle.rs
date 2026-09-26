use humfmt::{number_with, NumberOptions};

// XorShift64* PRNG for 0-dependency deterministic pseudo-random sequences
struct XorShift64(u64);
impl XorShift64 {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0 = self.0.wrapping_mul(0x2545F4914F6CDD1D);
        self.0
    }
    fn next_u128(&mut self) -> u128 {
        ((self.next() as u128) << 64) | (self.next() as u128)
    }
}

#[test]
fn oracle_u128_uncompacted() {
    let mut rng = XorShift64(1337);
    let opts = NumberOptions::new().compact(false).separators(false);
    
    for _ in 0..10_000 {
        let v = rng.next_u128();
        let hum = number_with(v, opts).to_string();
        let std = format!("{}", v);
        assert_eq!(hum, std, "Mismatch for {}", v);
    }
}

#[test]
fn oracle_u64_uncompacted() {
    let mut rng = XorShift64(0xDEADBEEF);
    let opts = NumberOptions::new().compact(false).separators(false);
    
    for _ in 0..50_000 {
        let v = rng.next();
        let hum = number_with(v, opts).to_string();
        let std = format!("{}", v);
        assert_eq!(hum, std, "Mismatch for {}", v);
    }
}
