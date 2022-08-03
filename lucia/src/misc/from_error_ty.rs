use core::fmt::Display;

/// Allows the usage of foreign error types into the codebase and resolves possible ambiguities
/// of internal associated types.
pub trait FromErrorTy {
  /// Any error structure that has the required bounds.
  type Error: Display + From<crate::Error>;
}

impl<T> FromErrorTy for &'_ T
where
  T: FromErrorTy,
{
  type Error = T::Error;
}

impl FromErrorTy for () {
  type Error = crate::Error;
}
