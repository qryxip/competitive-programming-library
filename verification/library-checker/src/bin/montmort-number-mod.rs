// verify-helper: PROBLEM https://judge.yosupo.jp/problem/montmort_number_mod

#[macro_use]
extern crate input as _;

use acl_modint::ModInt as Mint;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    Mint::set_modulus(m);
    let ans = &montmort_number::montmort::<Mint>(n)[1..];
    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" "),
    );
}
