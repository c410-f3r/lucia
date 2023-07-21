use crate::misc::AsyncTrait;
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;

/// Api definitions group different packages into a common namespace and define custom additional
/// logical through hooks.
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait Api {
  /// Any custom error structure that can be constructed from [crate::Error].
  type Error: From<crate::Error>;

  /// Fallible hook that is automatically called after sending any related request.
  #[inline]
  async fn after_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }

  /// Fallible hook that is automatically called before sending any related request.
  #[inline]
  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl Api for () {
  type Error = crate::Error;
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<T> Api for &mut T
where
  T: Api + AsyncTrait,
{
  type Error = T::Error;

  #[inline]
  async fn after_sending(&mut self) -> Result<(), Self::Error> {
    (**self).before_sending().await
  }

  #[inline]
  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    (**self).before_sending().await
  }
}
