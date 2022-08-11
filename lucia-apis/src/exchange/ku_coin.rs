//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! <https://docs.kucoin.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   misc::{CommonParams, Pair},
//!   network::{http::ReqParams, Transport},
//! };
//! use lucia_apis::{
//!   exchange::ku_coin::V1CurrenciesParams,
//!   misc::RequestManagerWrapper,
//! };
//!
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManagerWrapper::new(
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

use crate::misc::_MaxUrl;
use arrayvec::{ArrayString, ArrayVec};
use lucia::misc::{GenericTime, UrlPartsString};

#[derive(Debug, Default)]
pub struct KuCoin;

#[cfg(feature = "tokio-tungstenite")]
impl KuCoin {
  pub async fn tokio_tungstenite<DRSR>(
    bullet_server: &V1BulletServerRes,
    der_ser: DRSR,
    token: &str,
  ) -> crate::Result<
    lucia::Pair<
      lucia::RequestManager<Self, lucia::CommonParams<lucia::network::ws::ReqParams, ()>, DRSR>,
      lucia::network::TokioTungstenite,
    >,
  >
  where
    DRSR: Send,
  {
    use core::fmt::Write;
    use lucia::{CommonParams, Pair};
    use tokio_tungstenite::connect_async;

    let mut url = String::new();
    url.write_fmt(format_args!(
      "{endpoint}?token={token}&connectId={connect_id}&acceptUserMessage=true",
      endpoint = &*bullet_server.endpoint,
      connect_id = _timestamp()?
    ))?;
    let trans = connect_async(url.as_str()).await?.0;
    Ok(Pair::new(
      lucia::RequestManager::new(
        Self,
        CommonParams::new(
          lucia::network::ws::ReqParams { _url_parts: lucia::misc::UrlParts::from_url(url)? },
          (),
        ),
        der_ser,
      ),
      trans,
    ))
  }
}

#[derive(Debug)]
pub struct KuCoinUrls {
  pub v1_bullet_public: UrlPartsString,
  pub v1_currencies: UrlPartsString,
  pub v1_symbols: UrlPartsString,
  pub v2_currencies: UrlPartsString,
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
  pub endpoint: _MaxUrl,
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
