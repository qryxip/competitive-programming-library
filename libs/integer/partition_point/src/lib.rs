//! Computes the index of the partition point.
//!
//! # Examples
//!
//! ```
//! use partition_point::RangeBoundsExt as _;
//!
//! assert_eq!(1000, (0..1 << 30).partition_point(|n| n.to_string().len() <= 3));
//! assert_eq!(100, (0..100).partition_point(|_| true));
//! ```
//!
//! ```
//! use partition_point::SliceExt as _;
//!
//! assert_eq!(5, [0,    0,    0,    0,    0,    1,    1].partition_point(|&x| x == 0));
//! //             ^     ^     ^     ^     ^     ^     ^
//! //  index:     0     1     2     3     4     5     6
//! //  predicate: true  true  true  true  true  false false
//!
//! assert_eq!(7, [(),   (),   (),   (),   (),   (),   ()].partition_point(|_| true));
//! //             ^     ^     ^     ^     ^     ^     ^
//! //  index:     0     1     2     3     4     5     6
//! //  predicate: true  true  true  true  true  true  true
//! ```

use std::{
    fmt,
    ops::{Add, Bound, Div, RangeBounds, Sub},
};

pub trait RangeBoundsExt<T: PrimitiveInteger>: RangeBounds<T> {
    fn partition_point<P>(&self, mut pred: P) -> T
    where
        P: FnMut(T) -> bool,
    {
        let mut start = match self.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) if x > T::MIN_VALUE => x - T::ONE,
            _ => T::MIN_VALUE,
        };

        let mut end = match self.end_bound() {
            Bound::Included(&x) if x < T::MAX_VALUE => x + T::ONE,
            Bound::Excluded(&x) => x,
            _ if pred(T::MAX_VALUE) => panic!("the predicate is satisfied at {:?}", T::MAX_VALUE),
            _ => T::MAX_VALUE,
        };

        while start != end {
            let mid = start + (end - start) / (T::ONE + T::ONE);
            if pred(mid) {
                start = mid + T::ONE;
            } else {
                end = mid;
            }
        }
        start
    }
}

impl<T: PrimitiveInteger, R: RangeBounds<T>> RangeBoundsExt<T> for R {}

pub trait SliceExt {
    type Item;

    fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&Self::Item) -> bool;
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        (0..self.len()).partition_point(|i| pred(&self[i]))
    }
}

pub trait PrimitiveInteger:
    Copy + Ord + Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + fmt::Debug
{
    const ZERO: Self;
    const ONE: Self;
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;
}

macro_rules! impl_primitive_integer(($($ty:ty),*) => {
    $(
        impl PrimitiveInteger for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN_VALUE: Self = <$ty>::min_value();
            const MAX_VALUE: Self = <$ty>::max_value();
        }
    )*
});

impl_primitive_integer!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
