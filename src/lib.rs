//! My libraries for competitive programming.
//!
//! You cannot use this crate directly.

// TODO: expand crate-level docs with `custom-build` and `syn` crate.

macro_rules! re_export(($($name:ident),* $(,)?) => ($(pub mod $name { pub use ::$name::*; })*));

re_export!(partition_point, xorshift);

pub mod io {
    re_export!(buffered_print, input);
}

pub mod math {
    re_export!(montmort_number, tonelli_shanks);
}
