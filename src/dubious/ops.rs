//! forward implementations of std::ops traits.

use std::ops::{Add, Div, Mul, Sub};

use super::Dubious;

macro_rules! impl_bin_op {
    ($trait:ident, $method:ident) => {
        impl<T, U> $trait<Dubious<U>> for Dubious<T>
        where
            T: $trait<U>,
        {
            type Output = Dubious<T::Output>;

            #[inline]
            fn $method(self, rhs: Dubious<U>) -> Self::Output {
                self.zip_with(rhs, |(me, rhs)| me.$method(rhs))
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);
