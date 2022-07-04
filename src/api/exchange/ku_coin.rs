pub mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

use crate::{
  types::{MaxUrl, MaxUrlParts},
  utils::UrlParts,
};

#[derive(Debug)]
pub struct KuCoin {
  urls: KuCoinUrls,
}

impl KuCoin {
  #[inline]
  pub fn urls(&self) -> &KuCoinUrls {
    &self.urls
  }
}

impl crate::Api for KuCoin {
  #[inline]
  fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self {
      urls: KuCoinUrls {
        origin: origin.try_into()?,
        v1_bullet_public: UrlParts::from_origin_and_path(origin, "/api/v1/bullet-public")?,
        v1_currencies: UrlParts::from_origin_and_path(origin, "/api/v1/currencies")?,
        v1_symbols: UrlParts::from_origin_and_path(origin, "/api/v1/symbols")?,
        v2_currencies: UrlParts::from_origin_and_path(origin, "/api/v2/currencies")?,
      },
    })
  }

  #[inline]
  fn origin(&self) -> &MaxUrl {
    &self.urls.origin
  }
}

#[derive(Debug)]
pub struct KuCoinUrls {
  pub origin: MaxUrl,
  pub v1_bullet_public: MaxUrlParts,
  pub v1_currencies: MaxUrlParts,
  pub v1_symbols: MaxUrlParts,
  pub v2_currencies: MaxUrlParts,
}
