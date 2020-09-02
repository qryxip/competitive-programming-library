// verify-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

#[::cargo_equip::equip]
use ::__lib::{fenwick::AdditiveFenwickTree, input, output};

use std::io::Write as _;

fn main() {
    input! {
        n: usize,
        q: usize,
        r#as: [i64; n],
    }

    let mut fenwick = AdditiveFenwickTree::new(n);

    for (i, a) in r#as.into_iter().enumerate() {
        fenwick.plus(i, &a);
    }

    output::buf_print(|out| {
        macro_rules! println(($($tt:tt)*) => (writeln!(out, $($tt)*).unwrap()));
        for _ in 0..q {
            input!(kind: u32);
            match kind {
                0 => {
                    input!(p: usize, x: i64);
                    fenwick.plus(p, &x);
                }
                1 => {
                    input!(l: usize, r: usize);
                    println!("{}", fenwick.query(l..r));
                }
                _ => unreachable!(),
            }
        }
    });
}
