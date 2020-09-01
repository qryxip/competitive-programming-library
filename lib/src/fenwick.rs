use crate::algebraic::{Additive, Group, Monoid};

use std::{
    marker::PhantomData,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

pub type AdditiveFenwickTree<M> = FenwickTree<M, Additive>;

#[derive(Clone, Debug)]
pub struct FenwickTree<M, O> {
    nodes: Vec<M>,
    phantom: PhantomData<fn() -> O>,
}

impl<M: Clone + Monoid<O>, O> FenwickTree<M, O> {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![M::identity(); n],
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn query<I: FenwickTreeIndex<M, O>>(&self, index: I) -> M {
        index.query(&self.nodes)
    }

    pub fn plus(&mut self, i: usize, x: &M) {
        let mut i = i + 1;
        while i <= self.nodes.len() {
            self.nodes[i - 1] = self.nodes[i - 1].operate(x);
            i += 1 << i.trailing_zeros();
        }
    }
}

pub trait FenwickTreeIndex<M: Monoid<O>, O> {
    fn query(&self, nodes: &[M]) -> M;
}

impl<M: Group<O>, O> FenwickTreeIndex<M, O> for usize {
    fn query(&self, nodes: &[M]) -> M {
        let l = query(nodes, *self);
        let r = query(nodes, self + 1);
        r.operate(&l.two_sided_inverse())
    }
}

impl<M: Monoid<O>, O> FenwickTreeIndex<M, O> for RangeFull {
    #[inline]
    fn query(&self, nodes: &[M]) -> M {
        query(nodes, nodes.len())
    }
}

impl<M: Group<O>, O> FenwickTreeIndex<M, O> for RangeFrom<usize> {
    fn query(&self, nodes: &[M]) -> M {
        let l = query(nodes, self.start);
        let r = query(nodes, nodes.len());
        r.operate(&l.two_sided_inverse())
    }
}

impl<M: Monoid<O>, O> FenwickTreeIndex<M, O> for RangeTo<usize> {
    #[inline]
    fn query(&self, nodes: &[M]) -> M {
        query(nodes, self.end)
    }
}

impl<M: Monoid<O>, O> FenwickTreeIndex<M, O> for RangeToInclusive<usize> {
    #[inline]
    fn query(&self, nodes: &[M]) -> M {
        query(nodes, self.end + 1)
    }
}

impl<M: Group<O>, O> FenwickTreeIndex<M, O> for Range<usize> {
    fn query(&self, nodes: &[M]) -> M {
        let l = query(nodes, self.start);
        let r = query(nodes, self.end);
        r.operate(&l.two_sided_inverse())
    }
}

impl<M: Group<O>, O> FenwickTreeIndex<M, O> for RangeInclusive<usize> {
    fn query(&self, nodes: &[M]) -> M {
        let l = query(nodes, *self.start());
        let r = query(nodes, *self.end() + 1);
        r.operate(&l.two_sided_inverse())
    }
}

fn query<M: Monoid<O>, O>(nodes: &[M], len: usize) -> M {
    let mut acc = M::identity();
    let mut i = len;
    while i > 0 {
        acc = acc.operate(&nodes[i - 1]);
        i -= 1 << i.trailing_zeros();
    }
    acc
}
