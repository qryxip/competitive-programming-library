---
title: "qryxip-competitive-input (<code>input</code>)"
documentation_of: //crates/io/manifests/input/../../sourcefiles/input.rs
---
Provides `input!` macro.

# Example

```no_run
#[macro_use]
extern crate input as _;

fn main() {
    // https://atcoder.jp/contests/abc166/tasks/abc166_b

    input! {
        n: usize,
        ass: [[{ input::usize1 }]],
    }

    let _: usize = n;
    let _: Vec<Vec<usize>> = ass;
}
```
