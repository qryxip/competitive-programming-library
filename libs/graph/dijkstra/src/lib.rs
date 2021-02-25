//! Performs Dijkstra's algorithm.
//!
//! # Example
//!
//! ```
//! // [0] --1-> [1] --999-> [4]
//! //            |           ↑
//! //            1           1
//! //            ↓           |
//! //           [2] ---1--> [3]
//!
//! let start = 0;
//! let graph = vec![
//!     vec![(1, 1)],
//!     vec![(2, 1), (4, 999)],
//!     vec![(3, 1)],
//!     vec![(4, 1)],
//!     vec![],
//! ];
//!
//! let costs = dijkstra::costs(
//!     start,
//!     |i| graph[i].iter().copied(),
//!     vec![u64::max_value(); graph.len()],
//!     |graph, i| &mut graph[i],
//! );
//!
//! assert_eq!([0, 1, 2, 3, 4], *costs);
//! ```

use std::{
    collections::BinaryHeap,
    iter::{self, Sum},
    ops::Add,
};

/// Computes costs for each node.
pub fn costs<V, Es, Vs, Ws, WsI, W>(start: V, mut neighbors: Es, mut costs: Ws, mut cost: WsI) -> Ws
where
    V: Copy + Ord,
    Es: FnMut(V) -> Vs,
    Vs: IntoIterator<Item = (V, W)>,
    WsI: FnMut(&mut Ws, V) -> &mut W,
    W: Copy + Ord + Add<Output = W> + Sum,
{
    *cost(&mut costs, start) = iter::empty().sum();
    let queue = &mut BinaryHeap::from(vec![(iter::empty().sum(), start)]);
    while let Some((current_cost, current_node)) = queue.pop() {
        if *cost(&mut costs, current_node) < current_cost {
            continue;
        }
        for (next_node, cost_delta) in neighbors(current_node) {
            let next_cost = current_cost + cost_delta;
            if next_cost < *cost(&mut costs, next_node) {
                *cost(&mut costs, next_node) = next_cost;
                queue.push((next_cost, next_node));
            }
        }
    }
    costs
}
