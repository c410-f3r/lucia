//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! <https://docs.kucoin.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{exchange::ku_coin::KuCoin, misc::PackagesAux};
//!
//! let mut pkgs_aux =
//!   PackagesAux::from_minimum(KuCoin::new(None)?, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.v1_currencies().build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod ku_coin_credentials;
mod pkg;

use crate::misc::PackagesAux;
use arrayvec::ArrayString;
use core::time::Duration;
pub use ku_coin_credentials::*;
use lucia::{
  misc::{GenericTime, RequestLimit, RequestThrottling},
  network::HttpParams,
  Api,
};
pub use pkg::*;

pub(crate) type KuCoinHttpPackagesAux<DRSR> = PackagesAux<KuCoin, DRSR, HttpParams>;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
pub struct KuCoin {
  credentials: Option<KuCoinCredentials>,
  orders_rt: RequestThrottling,
}

impl KuCoin {
  /// Creates an instance with or without credentials used in private endpoints.
  pub fn new(credentials: Option<KuCoinCredentials>) -> crate::Result<Self> {
    Ok(Self {
      credentials,
      orders_rt: RequestThrottling::from_rl(RequestLimit::new(45, Duration::from_secs(3))?)?,
    })
  }

  /// Returns a ready-to-use pair containing "tokio_tungstenite" as the transport for WebSocket
  /// connections.
  ///
  /// If private, then a token should be provided. Otherwise the connection will be assumed as
  /// public.
  #[cfg(feature = "tokio-tungstenite")]
  pub async fn tokio_tungstenite<'drsr, DRSR>(
    &mut self,
    bullet_url: &str,
    drsr: &'drsr mut DRSR,
    token_opt: Option<&str>,
  ) -> crate::Result<
    lucia::misc::Pair<
      PackagesAux<KuCoin, &'drsr mut DRSR, lucia::network::WsParams>,
      lucia::network::transport::TokioTungstenite,
    >,
  > {
    use core::fmt::Write;
    use lucia::misc::Pair;
    use tokio_tungstenite::connect_async;

    let id = _timestamp()?;
    let mut url = String::new();
    let rslt = if let Some(token) = token_opt {
      url
        .write_fmt(format_args!("{bullet_url}?token={token}&connectId={id}&acceptUserMessage=true"))
    } else {
      url.write_fmt(format_args!("{bullet_url}?connectId={id}&acceptUserMessage=true"))
    };
    rslt.map_err(lucia::Error::from)?;
    Ok(Pair::new(
      PackagesAux::from_minimum(KuCoin::new(None)?, drsr, lucia::network::WsParams::default()),
      connect_async(url.as_str()).await.map_err(lucia::Error::from)?.0,
    ))
  }
}

impl Api for KuCoin {
  type Error = crate::Error;
}

/// Almost all responses are wrapped in "code" and "data".
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct GenericDataResponse<T> {
  /// System code
  pub code: ArrayString<8>,
  /// Actual data
  pub data: T,
}

pub(crate) fn _timestamp() -> crate::Result<i64> {
  Ok(GenericTime::now()?.timestamp()?.as_millis().try_into()?)
}
