pub trait Monoid<O>: Eq {
    fn identity() -> Self;
    fn operate(&self, rhs: &Self) -> Self;
}

pub trait Group<O>: Monoid<O> {
    fn two_sided_inverse(&self) -> Self;
}

#[derive(Debug)]
pub enum Additive {}

macro_rules! impl_monoid {
    ($($ty:ty),*) => {
        $(
            impl Monoid<Additive> for $ty {
                #[inline]
                fn identity() -> Self {
                    0
                }

                #[inline]
                fn operate(&self, rhs: &Self) -> Self {
                    self + rhs
                }
            }
        )*
    }
}

impl_monoid!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_group {
    ($($ty:ty),*) => {
        $(
            impl Group<Additive> for $ty {
                #[inline]
                fn two_sided_inverse(&self) -> Self {
                    -self
                }
            }
        )*
    }
}

impl_group!(i8, i16, i32, i64, i128, isize);
