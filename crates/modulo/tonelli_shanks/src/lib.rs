//! Performs Tonelli–Shanks algorithm.
//!
//! # Example
//!
//! ```
//! # use __acl_modint as acl_modint;
//! use acl_modint::ModInt1000000007 as Mint;
//! use tonelli_shanks::ModIntBaseExt as _;
//!
//! assert_eq!(Some(82_681_419.into()), Mint::new(42).sqrt());
//! ```

mod extern_crates {
    pub(super) extern crate __acl_modint as acl_modint;
    pub(super) extern crate __xorshift as xorshift;
}

use self::extern_crates::{acl_modint::ModIntBase, xorshift::Xorshift64};

/// Performs Tonelli–Shanks algorithm.
pub trait ModIntBaseExt: ModIntBase {
    /// Performs Tonelli–Shanks algorithm.
    fn sqrt(self) -> Option<Self> {
        let q = u64::from((Self::modulus() - 1) >> (Self::modulus() - 1).trailing_zeros());
        let s = (Self::modulus() - 1).trailing_zeros();

        let z = {
            let mut xor64 = Xorshift64::new();
            loop {
                let z = Self::new(xor64.next_u64() % u64::from(Self::modulus()));
                if z.pow(((Self::modulus() - 1) / 2).into()).val() == Self::modulus() - 1 {
                    break z;
                }
            }
        };

        let mut m = s;
        let mut c = z.pow(q);
        let mut t = self.pow(q);
        let mut r = self.pow((q + 1) / 2);

        'ret: loop {
            if t.val() == 0 {
                break Some(Self::new(0));
            }

            if t.val() == 1 {
                break Some(r);
            }

            let i = {
                let mut acc = t * t;
                let mut i = 1;
                loop {
                    if i == m {
                        break 'ret None;
                    }
                    if acc.val() == 1 {
                        break i;
                    }
                    acc *= acc;
                    i += 1;
                }
            };

            let b = {
                let mut b = c;
                for _ in 0..m - i - 1 {
                    b *= b;
                }
                b
            };

            m = i;
            c = b * b;
            t *= b * b;
            r *= b;
        }
    }
}

impl<Z: ModIntBase> ModIntBaseExt for Z {}
