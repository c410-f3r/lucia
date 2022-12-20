/// Api definitions group different packages into a common namespace and define custom additional
/// logical through hooks.
pub trait Api {
  /// Any custom error structure that can be constructed from [crate::Error].
  type Error: From<crate::Error>;

  /// Fallible hook that is automatically called before sending any related request.
  #[inline]
  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl Api for () {
  type Error = crate::Error;
}

impl<T> Api for &mut T
where
  T: Api,
{
  type Error = T::Error;

  #[inline]
  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    (**self).before_sending().await
  }
}
