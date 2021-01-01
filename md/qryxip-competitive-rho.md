---
title: "qryxip-competitive-rho (<code>rho</code>)"
documentation_of: //crates/prime/manifests/rho/../../sourcefiles/rho.rs
---
Performs Pollard's rho algorithm.

# Example

```rust
assert_eq!([1_162_193, 1_347_377], *rho::factorize(1_565_912_117_761));
```
