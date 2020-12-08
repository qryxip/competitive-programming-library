//! Crates for competitive programming.
//!
//! This crate is not intended to be used directly.

// TODO: expand crate-level docs with `custom-build` and `syn` crate.

macro_rules! re_export(($($name:ident),* $(,)?) => ($(pub mod $name { pub use ::$name::*; })*));

pub mod integer {
    re_export!(gcd, montmort_number, partition_point, xorshift);
}

pub mod io {
    re_export!(buffered_print, input);
}

pub mod modulo {
    re_export!(tonelli_shanks);
}

pub mod prime {
    re_export!(miller_rabin, rho);
}
