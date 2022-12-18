//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! <https://docs.kucoin.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{exchange::ku_coin::KuCoin, misc::PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(KuCoin::new(None)?, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.v1_get_currencies().build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod ku_coin_credentials;
mod pkg;

use crate::misc::PkgsAux;
use core::{fmt::Write, time::Duration};
pub use ku_coin_credentials::*;
use lucia::{
  misc::{GenericTime, Pair, RequestLimit, RequestThrottling},
  network::{transport::Transport, WebSocket, WsParams},
  Api,
};
pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(PkgsAux), transport(http, ws))]
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

  /// Returns a ready-to-use pair containing WebSocket transport.
  ///
  /// If private, then a token should be provided. Otherwise the connection will be assumed as
  /// public.
  pub async fn web_socket<DRSR, T>(
    bullet_url: &str,
    bytes: &mut Vec<u8>,
    drsr: DRSR,
    token_opt: Option<&str>,
  ) -> crate::Result<Pair<KuCoinWsPkgsAux<DRSR>, T>>
  where
    T: Transport<DRSR, Params = WsParams> + WebSocket,
  {
    let id = GenericTime::now()?.timestamp()?.as_millis();
    let mut url = String::new();
    let rslt = if let Some(token) = token_opt {
      url
        .write_fmt(format_args!("{bullet_url}?token={token}&connectId={id}&acceptUserMessage=true"))
    } else {
      url.write_fmt(format_args!("{bullet_url}?connectId={id}&acceptUserMessage=true"))
    };
    rslt.map_err(lucia::Error::from)?;
    let mut trans = T::from_url(url.as_str()).await?;
    let before_len = bytes.len();
    let _len = trans.receive_with_buffer(bytes).await;
    bytes.truncate(before_len);
    Ok(Pair::new(PkgsAux::from_minimum(KuCoin::new(None)?, drsr, WsParams::default()), trans))
  }
}

impl Api for KuCoin {
  type Error = crate::Error;
}
