//! Crates for competitive programming.
//!
//! This crate is not intended to be used directly.

// TODO: expand crate-level docs with `custom-build` and `syn` crate.

macro_rules! re_export(($($name:ident),* $(,)?) => ($(pub mod $name { pub use ::$name::*; })*));

re_export!(partition_point, xorshift);

pub mod io {
    re_export!(buffered_print, input);
}

pub mod math {
    re_export!(gcd, miller_rabin, montmort_number, rho, tonelli_shanks);
}
