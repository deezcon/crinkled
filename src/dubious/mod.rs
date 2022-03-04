//! Dubious values.
//!
//! [`Dubious`] is a zero-cost wrapper around a value for use when:
//! - There exists some invalid states of the contained value.
//! - Those invalid states *must* be checked before the value can be used
//!   elsewhere.
//! - There is nothing inherently incorrect with operating on a value with an
//!   invalid state.
//!
//! TODO: give a bit more on type-theory explination?
//! For my type theorists out there: [`Dubious`] is a monad. Within a
//! [`Dubious`] you can treat ts value as if it were valid.
//!
//! More technically, a [`Dubious<T>`] is an otherwise valid rust value of type
//! `T` with additional, *fallible* constraints that must be verified before it
//! can be regarded as an acceptable value of type `T`, but operating on `T`s
//! that are ultimately invalid is acceptable within certain contexts.
//!
//! Another way to look at [`Dubious`] is as a lazy [`Result`] or [`Option`]
//! that you can operate on as needed, and perform any computation needed to
//! turn it into a [`Result`] or [`Option`] at the end. Indeed, many methods
//! of [`Dubious`] return [`Result`]s and [`Option`]s, or use them internally.
//!
//! An example usage of [`Dubious`] is when working with floating point numbers
//! in an application that cannot tolerate NaN or infinite values. Checking the
//! result of each step could add an undesireable performance hit, and manually
//! checking at various stages is error prone (and possibly tediuos). Working
//! with a [`Dubious<T>`] guarantees any receiver expecting a `T` will get a
//! valid `T` (in this case a float that must be finite).
//!
//! To make the most use of the [`Dubious`] type, wrap the scrutinee in a
//! newtype and use that as the expected type throughout your code. In our float
//! example, we might create a `Scalar` type that wraps `f64`.

#[cfg(feature = "forward-ops")]
pub mod ops;
pub mod validate;

pub use validate::Validate;

/// The `Dubious` type. See [the module level documentation](self) for more.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Dubious<T>(T);

impl<T> Dubious<T> {
    #[inline]
    pub const fn new(value: T) -> Dubious<T> {
        Dubious(value)
    }

    /// Maps a `Dubious<T>` to `Dubious<U>` by applying a function to a
    /// contained value.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Dubious<U>
    where
        F: FnOnce(T) -> U,
    {
        Dubious(f(self.0))
    }

    /// Validate the `Dubious<T>` with the function.
    ///
    /// Use this if `T` does not implement the [`Validate`] trait, or if you
    /// need to handle validation differently than the existing implementation.
    #[inline]
    pub fn validate_with<F, E>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(T) -> Result<T, E>,
    {
        f(self.0)
    }

    /// Zips `self` with another `Dubious`.
    ///
    /// ---
    /// *Neither `self` nor `other` is validated.*
    #[inline]
    pub fn zip<U>(self, other: Dubious<U>) -> Dubious<(T, U)> {
        Dubious((self.0, other.0))
    }

    /// Zips `self` and another `Dubious` with function `f`. `zip_with` is
    /// semantically equivalent [`zip`]ing, and then [`map`]ing as in
    /// `self.zip(other).map(f)`.
    ///
    /// ---
    /// *Neither `self` nor `other` is validated.*
    ///
    /// [`zip`]: Dubious::zip
    /// [`map`]: Dubious::map
    #[inline]
    pub fn zip_with<U, F, R>(self, other: Dubious<U>, f: F) -> Dubious<R>
    where
        F: FnOnce((T, U)) -> R,
    {
        Dubious(f((self.0, other.0)))
    }
}

impl<T> Dubious<Dubious<T>> {
    /// Converts from `Dubious<Dubious<T>>` to `Dubious<T>`.
    ///
    /// Flattening only removes one level of nesting at a time.
    #[inline]
    pub fn flatten(self) -> Dubious<T> {
        self.0
    }
}

impl<T> Dubious<Option<T>> {
    /// Converts from `Dubious<Option<T>>` to `Option<Dubious<T>>`.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use crinkled::dubious::Dubious;
    ///
    /// let foo = Dubious::new(Some("foo"));
    /// assert_eq!(foo.invert(), Some(Dubious::new("foo")));
    ///
    /// let bar: Dubious<Option<&str>> = Dubious::new(None);
    /// assert_eq!(bar.invert(), None);
    /// ```
    /// ---
    /// *There is currently no method for the inverse `Option<Dubious<T>>` to
    /// `Dubious<Option<T>>`.*
    // --- dev notes ---
    // to add the inverse, either support for inherent impls on foreign
    // types needs to be added to rust, or something like a marker trait needs
    // to be added.
    #[inline]
    pub fn invert(self) -> Option<Dubious<T>> {
        self.0.map(|x| Dubious(x))
    }
}

impl<T> Dubious<T>
where
    Dubious<T>: Validate<T>,
{
    /// Zips `self` with another `Dubious`.
    ///
    /// If `self.ok()` is `Some(s)` and `other.ok()` is `Some(o)`, this metod
    /// returns `Some(Dubious((s, o)))`. Otherwise, `None` is returned. This
    /// method does not validate `self` or `other`.
    pub fn zip_ok<U>(self, other: Dubious<U>) -> Option<Dubious<(T, U)>>
    where
        Dubious<U>: Validate<U>,
    {
        self.ok().zip(other.ok()).map(|t| Dubious(t))
    }
}

impl<T> Validate<T> for Dubious<T>
where
    T: Validate<T>,
{
    type Error = T::Error;

    #[inline]
    fn validate(self) -> Result<T, Self::Error> {
        self.0.validate()
    }
}

impl<T> From<T> for Dubious<T> {
    #[inline]
    fn from(x: T) -> Dubious<T> {
        Dubious(x)
    }
}

impl<T: PartialEq> PartialEq<T> for Dubious<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

// /// The `Almost` type. See [the module level documentation](self) for more.
// #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
// pub struct Almost<T> {
//     value: T,
// }
