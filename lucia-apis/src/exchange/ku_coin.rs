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
//! let _ = pkgs_aux.v1_get_currencies().build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod ku_coin_credentials;
mod pkg;

use core::time::Duration;
pub use ku_coin_credentials::*;
use lucia::{
  misc::{RequestLimit, RequestThrottling},
  Api,
};
pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(crate::misc::PackagesAux), transport(http, ws))]
pub struct KuCoin {
  credentials: Option<KuCoinCredentials>,
  orders_rt: RequestThrottling,
  order_book_rt: RequestThrottling,
}

impl KuCoin {
  /// Creates an instance with or without credentials used in private endpoints.
  pub fn new(credentials: Option<KuCoinCredentials>) -> crate::Result<Self> {
    Ok(Self {
      credentials,
      order_book_rt: RequestThrottling::from_rl(RequestLimit::new(30, Duration::from_secs(3))?)?,
      orders_rt: RequestThrottling::from_rl(RequestLimit::new(45, Duration::from_secs(3))?)?,
    })
  }

  /// Returns a ready-to-use pair containing "tokio_tungstenite" as the transport for WebSocket
  /// connections.
  ///
  /// If private, then a token should be provided. Otherwise the connection will be assumed as
  /// public.
  #[cfg(feature = "tokio-tungstenite")]
  pub async fn tokio_tungstenite<DRSR>(
    bullet_url: &str,
    drsr: DRSR,
    token_opt: Option<&str>,
  ) -> crate::Result<
    lucia::misc::Pair<KuCoinWsPkgsAux<DRSR>, lucia::network::transport::TokioTungstenite>,
  > {
    use core::fmt::Write;
    use futures::stream::StreamExt;
    use lucia::misc::{GenericTime, Pair};
    use tokio_tungstenite::connect_async;

    let id = GenericTime::now()?.timestamp()?.as_millis();
    let mut url = String::new();
    let rslt = if let Some(token) = token_opt {
      url
        .write_fmt(format_args!("{bullet_url}?token={token}&connectId={id}&acceptUserMessage=true"))
    } else {
      url.write_fmt(format_args!("{bullet_url}?connectId={id}&acceptUserMessage=true"))
    };
    rslt.map_err(lucia::Error::from)?;
    let mut trans = connect_async(url.as_str()).await.map_err(lucia::Error::from)?.0;
    let _ = trans.next().await;
    Ok(Pair::new(
      crate::misc::PackagesAux::from_minimum(
        KuCoin::new(None)?,
        drsr,
        lucia::network::WsParams::default(),
      ),
      trans,
    ))
  }
}

impl Api for KuCoin {
  type Error = crate::Error;
}
