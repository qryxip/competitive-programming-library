---
title: "qryxip-competitive-tonelli-shanks (<code>tonelli_shanks</code>)"
documentation_of: //crates/modulo/manifests/tonelli_shanks/../../sourcefiles/tonelli_shanks.rs
---
Performs Tonelli–Shanks algorithm.

# Example

```rust
# use __acl_modint as acl_modint;
use acl_modint::ModInt1000000007 as Mint;
use tonelli_shanks::ModIntBaseExt as _;

assert_eq!(Some(82_681_419.into()), Mint::new(42).sqrt());
```
