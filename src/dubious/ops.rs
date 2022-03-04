//! forward implementations of std::ops traits.

use super::{Dubious, DubiousMarker};

macro_rules! impl_bin_op {
    ($trait:ident, $method:ident) => {
        impl<T, U> std::ops::$trait<Dubious<U>> for Dubious<T>
        where
            T: std::ops::$trait<U>,
            <T as std::ops::$trait<U>>::Output: DubiousMarker,
        {
            type Output = T::Output;

            #[inline]
            fn $method(self, rhs: Dubious<U>) -> Self::Output {
                self.0.$method(rhs.0)
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);
