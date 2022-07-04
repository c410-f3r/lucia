pub mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

use crate::types::MaxUrl;

#[derive(Debug)]
pub struct Covid19 {
  pub origin: MaxUrl,
}

impl crate::Api for Covid19 {
  #[inline]
  fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self { origin: origin.try_into()? })
  }

  #[inline]
  fn origin(&self) -> &MaxUrl {
    &self.origin
  }
}
