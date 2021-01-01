---
title: "qryxip-competitive-dijkstra (<code>dijkstra</code>)"
documentation_of: //crates/graph/manifests/dijkstra/../../sourcefiles/dijkstra.rs
---
Performs Dijkstra's algorithm.

# Example

```rust
// [0] --1-> [1] --999-> [4]
//            |           ↑
//            1           1
//            ↓           |
//           [2] ---1--> [3]

let start = 0;
let graph = vec![
    vec![(1, 1)],
    vec![(2, 1), (4, 999)],
    vec![(3, 1)],
    vec![(4, 1)],
    vec![],
];

let costs = dijkstra::costs(
    start,
    |i| graph[i].iter().copied(),
    vec![u64::max_value(); graph.len()],
    |graph, i| &mut graph[i],
);

assert_eq!([0, 1, 2, 3, 4], *costs);
```
