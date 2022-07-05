pub mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

use crate::{api::Api, types::MaxUrl};

#[derive(Debug)]
pub struct Covid19 {
  pub origin: MaxUrl,
}

impl Api for Covid19 {
  type Aux = ();

  #[inline]
  fn new(origin: &str, _: Self::Aux) -> crate::Result<Self> {
    Ok(Self { origin: origin.try_into()? })
  }

  #[inline]
  fn origin(&self) -> &MaxUrl {
    &self.origin
  }
}
