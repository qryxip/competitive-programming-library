#[macro_use]
extern crate fastout as _;
#[macro_use]
extern crate input as _;

use acl_modint::ModInt;
use tonelli_shanks::ModIntBaseExt as _;

#[fastout]
fn main() {
    input! {
        yps: [(u32, u32)],
    }

    for (y, p) in yps {
        ModInt::set_modulus(p);
        if let Some(sqrt) = ModInt::new(y).sqrt() {
            println!("{}", sqrt);
        } else {
            println!("-1");
        }
    }
}
