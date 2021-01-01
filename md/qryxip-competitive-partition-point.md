---
title: "qryxip-competitive-partition-point (<code>partition_point</code>)"
documentation_of: //crates/integer/manifests/partition_point/../../sourcefiles/partition_point.rs
---
Computes the index of the partition point.

# Examples

```rust
use partition_point::RangeBoundsExt as _;

assert_eq!(1000, (0..1 << 30).partition_point(|n| n.to_string().len() <= 3));
assert_eq!(100, (0..100).partition_point(|_| true));
```

```rust
use partition_point::SliceExt as _;

assert_eq!(5, [0, 0, 0, 0, 0, 1, 1].partition_point(|&x| x == 0));
```
