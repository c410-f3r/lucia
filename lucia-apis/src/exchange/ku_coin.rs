//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! <https://docs.kucoin.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{exchange::ku_coin::KuCoin, misc::PackagesAux};
//!
//! let mut pkgs_aux = PackagesAux::from_minimum(KuCoin, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.v1_currencies().build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use crate::misc::PackagesAux;
use arrayvec::ArrayString;
use lucia::{misc::GenericTime, network::HttpParams};
pub use pkg::*;

pub(crate) type KuCoinHttpPackagesAux<DRSR> = PackagesAux<KuCoin, DRSR, HttpParams>;

#[derive(Debug)]
#[doc = _generic_dummy_api_doc!()]
pub struct KuCoin;

#[cfg(feature = "tokio-tungstenite")]
impl KuCoin {
  /// Returns a ready-to-use pair containing "tokio_tungstenite" as the transport for WebSocket
  /// connections.
  ///
  /// If private, then a token should be provided. Otherwise the connection will be assumed as
  /// public.
  pub async fn tokio_tungstenite<DRSR>(
    bullet_server: &V1BulletInstanceServersResData,
    drsr: DRSR,
    token_opt: Option<&str>,
  ) -> crate::Result<
    lucia::misc::Pair<
      PackagesAux<Self, DRSR, lucia::network::WsParams>,
      lucia::network::transport::TokioTungstenite,
    >,
  > {
    use core::fmt::Write;
    use lucia::misc::Pair;
    use tokio_tungstenite::connect_async;

    let endpoint = &*bullet_server.endpoint;
    let id = _timestamp()?;
    let mut url = String::new();
    let rslt = if let Some(token) = token_opt {
      url.write_fmt(format_args!("{endpoint}?token={token}&connectId={id}&acceptUserMessage=true"))
    } else {
      url.write_fmt(format_args!("{endpoint}?connectId={id}&acceptUserMessage=true"))
    };
    rslt.map_err(lucia::Error::from)?;
    Ok(Pair::new(
      PackagesAux::from_minimum(Self, drsr, lucia::network::WsParams::default()),
      connect_async(url.as_str()).await.map_err(lucia::Error::from)?.0,
    ))
  }
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
