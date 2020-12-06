// verify-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_mod

// Uncomment this line if you don't use your libraries. (`--check` still works)
//#![cfg_attr(cargo_equip, cargo_equip::skip)]

#[macro_use]
extern crate input as _;

use acl_modint::ModInt;
use std::io::Write as _;
use tonelli_shanks::ModIntBaseExt as _;

fn main() {
    input! {
        yps: [(u32, u32)],
    }

    buffered_print::buf_print(|out| {
        macro_rules! println(($($tt:tt)*) => (writeln!(out, $($tt)*).unwrap()));
        for (y, p) in yps {
            ModInt::set_modulus(p);
            if let Some(sqrt) = ModInt::new(y).sqrt() {
                println!("{}", sqrt);
            } else {
                println!("-1");
            }
        }
    });
}
