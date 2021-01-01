//! Xorshift.
//!
//! # Example
//!
//! ```
//! use xorshift::Xorshift64;
//!
//! let mut xor64 = Xorshift64::new();
//! let _: u64 = xor64.next_u64();
//! let _: u64 = xor64.next_u64();
//! let _: u64 = xor64.next_u64();
//! ```

/// 64-bit Xorshift.
#[derive(Copy, Clone, Debug)]
pub struct Xorshift64 {
    x: u64,
}

impl Xorshift64 {
    pub fn new() -> Self {
        Self {
            x: 88_172_645_463_325_252,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.x ^= self.x << 13;
        self.x ^= self.x >> 7;
        self.x ^= self.x << 17;
        self.x
    }
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Xorshift64 {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.next_u64())
    }
}
