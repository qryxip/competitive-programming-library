use std::{
    collections::BinaryHeap,
    iter::{self, Sum},
    ops::Add,
};

pub fn costs<V, Es, Vs, Ws, WsI, W>(start: V, mut neighbors: Es, mut costs: Ws, mut cost: WsI) -> Ws
where
    V: Copy + Ord,
    Es: FnMut(V) -> Vs,
    Vs: IntoIterator<Item = (V, W)>,
    WsI: FnMut(&mut Ws, V) -> &mut W,
    W: Copy + Ord + Add<Output = W> + Sum,
{
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
