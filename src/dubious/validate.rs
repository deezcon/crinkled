/// Fallible validation of values.
///
/// `Ok` is the type returned when validation is successful. Note that `Ok` is
/// `Self` by default, but this is not mandatory.
pub trait Validate<Ok = Self> {
    /// The type returned in the event of a validation error.
    type Error;

    /// Performs the validation.
    fn validate(self) -> Result<Ok, Self::Error>;

    /// Converts `self` into an [`Option<Ok>`] by consuming and validating
    /// `self`, and discarding the error, if any.
    ///
    /// Returns [`Some`] if `self` is valid, otherwise [`None`].
    #[inline]
    fn ok(self) -> Option<Ok>
    where
        Self: Sized, // TODO: remove this restriction
    {
        self.validate().ok()
    }

    /// Converts `self` into an [`Option<E>`] by consuming and validating
    /// `self`, and discarding the success value, if any.
    ///
    /// Returns [`Some`] if `self` is invalid, otherwise [`None`].
    #[inline]
    fn err(self) -> Option<Self::Error>
    where
        Self: Sized, // TODO: remove this restriction
    {
        self.validate().err()
    }
}
