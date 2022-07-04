use crate::utils::RequestThrottling;

/// Uses [crate::utils::RequestThrottling] to manage any given `T` and stop processing where
/// applicable.
#[derive(Debug)]
pub struct ManagedRequestResource<T>(T);

impl<T> ManagedRequestResource<T> {
  /// Returns the internal value or awaits until fulfillment. See [crate::utils::RequestCounter]
  /// for more information.
  #[inline]
  pub async fn limited<'this, 'rt, 'rslt>(&'this self, rt: &'rt mut RequestThrottling) -> &'rslt T
  where
    'this: 'rslt,
  {
    rt.rc.update_params(&rt.rl).await;
    &self.0
  }

  /// Returns the internal value regardless if processing should stop. Good in environments where
  /// `T` is used in a context without thresholds.
  #[inline]
  pub fn unlimited(&self) -> &T {
    &self.0
  }
}

impl<T> From<T> for ManagedRequestResource<T> {
  #[inline]
  fn from(from: T) -> Self {
    Self(from)
  }
}
