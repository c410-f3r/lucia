//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! <https://docs.kucoin.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::exchange::ku_coin::V1CurrenciesParams,
//!   network::{http::ReqParams, Transport},
//!   CommonParams, Pair, RequestManager
//! };
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManager::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.v1_currencies();
//! let _res = trans.send_and_retrieve(&mut rm, &req, V1CurrenciesParams::new()).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

use crate::{
  types::{MaxUrl, MaxUrlParts},
  utils::GenericTime,
};
use arrayvec::{ArrayString, ArrayVec};

#[derive(Debug, Default)]
pub struct KuCoin;

#[cfg(feature = "tokio-tungstenite")]
impl KuCoin {
  pub async fn tokio_tungstenite<DRSR>(
    bullet_server: &V1BulletServerRes,
    der_ser: DRSR,
    token: &str,
  ) -> crate::Result<
    crate::Pair<
      crate::RequestManager<Self, crate::CommonParams<crate::network::ws::ReqParams, ()>, DRSR>,
      crate::network::TokioTungstenite,
    >,
  >
  where
    DRSR: Send,
  {
    use crate::{CommonParams, Pair};
    use core::fmt::Write;
    use tokio_tungstenite::connect_async;

    let mut url = ArrayString::new();
    url.write_fmt(format_args!(
      "{endpoint}?token={token}&connectId={connect_id}&acceptUserMessage=true",
      endpoint = &*bullet_server.endpoint,
      connect_id = _timestamp()?
    ))?;
    Ok(Pair::new(
      crate::RequestManager::new(
        Self,
        CommonParams::new(
          crate::network::ws::ReqParams { _url_parts: crate::utils::UrlParts::from_url(url)? },
          (),
        ),
        der_ser,
      ),
      connect_async(url.as_str()).await?.0,
    ))
  }
}

#[derive(Debug)]
pub struct KuCoinUrls {
  pub v1_bullet_public: MaxUrlParts,
  pub v1_currencies: MaxUrlParts,
  pub v1_symbols: MaxUrlParts,
  pub v2_currencies: MaxUrlParts,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct GenericDataResponse<T> {
  pub code: ArrayString<8>,
  pub data: T,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1BulletServerRes {
  pub encrypt: bool,
  pub endpoint: MaxUrl,
  pub ping_interval: u64,
  pub ping_timeout: u64,
  pub protocol: ArrayString<12>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1BulletRes {
  pub instance_servers: ArrayVec<V1BulletServerRes, 4>,
  pub token: ArrayString<200>,
}

pub(crate) fn _timestamp() -> crate::Result<i64> {
  Ok(GenericTime::now()?.timestamp()?.as_millis().try_into()?)
}
