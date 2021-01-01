---
title: "qryxip-competitive-xorshift (<code>xorshift</code>)"
documentation_of: //crates/integer/manifests/xorshift/../../sourcefiles/xorshift.rs
---
Xorshift.

# Example

```rust
use xorshift::Xorshift64;

let mut xor64 = Xorshift64::new();
let _: u64 = xor64.next_u64();
let _: u64 = xor64.next_u64();
let _: u64 = xor64.next_u64();
```
