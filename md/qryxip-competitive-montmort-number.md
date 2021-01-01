---
title: "qryxip-competitive-montmort-number (<code>montmort_number</code>)"
documentation_of: //crates/integer/manifests/montmort_number/../../sourcefiles/montmort_number.rs
---
Computes Montmort numbers.

# Example

```rust
assert_eq!(
    vec![0usize, 0, 1, 2, 9, 44, 265, 1854, 14833, 133_496, 1_334_961],
    montmort_number::montmort(10),
);
```
