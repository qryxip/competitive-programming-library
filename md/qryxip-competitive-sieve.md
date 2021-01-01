---
title: "qryxip-competitive-sieve (<code>sieve</code>)"
documentation_of: //crates/prime/manifests/sieve/../../sourcefiles/sieve.rs
---
A prime sieve.

# Example

```rust
use sieve::Sieve;

let sieve = Sieve::new(30);

assert!(sieve.is_prime(2));
assert_eq!([(2, 2), (3, 2), (5, 2)], *sieve.factorize(900));
```
