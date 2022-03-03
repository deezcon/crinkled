//! forward implementations of std::ops traits.

use super::Dubious;
// use std::ops::{Add, Div, Mul, Sub};

// macro_rules! impl_bin_op {
//     ($trait:ident, $method:ident) => {
//         impl<T, U> $trait<Dubious<U>> for Dubious<T>
//         where
//             T: $trait<U>,
//         {
//             type Output = Dubious<T::Output>;

//             #[inline]
//             fn $method(self, rhs: Dubious<U>) -> Self::Output {
//                 self.zip_with(rhs, |(me, rhs)| me.$method(rhs))
//             }
//         }
//     };
// }

// impl_bin_op!(Add, add);
// impl_bin_op!(Sub, sub);
// impl_bin_op!(Mul, mul);
// impl_bin_op!(Div, div);

macro_rules! decl_bin_op {
    ($trait:ident, $method:ident) => {
        pub trait $trait<Rhs = Self> {
            type Output;

            fn $method(self, rhs: Rhs) -> Self::Output;
        }
    };
}

decl_bin_op!(Add, add);
decl_bin_op!(Sub, sub);
decl_bin_op!(Mul, mul);
decl_bin_op!(Div, div);

macro_rules! impl_bin_op {
    ($trait:ident, $method:ident) => {
        impl<T, U> std::ops::$trait<Dubious<U>> for Dubious<T>
        where
            T: std::ops::$trait<U>,
        {
            type Output = Dubious<T::Output>;

            #[inline]
            fn $method(self, rhs: Dubious<U>) -> Self::Output {
                Dubious(self.0.$method(rhs.0))
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);
