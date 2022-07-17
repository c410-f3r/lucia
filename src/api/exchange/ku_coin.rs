//! KuCoin is a global cryptocurrency exchange for numerous digital assets and cryptocurrencies.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::exchange::ku_coin::V1CurrenciesParams,
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.v1_currencies();
//! let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, V1CurrenciesParams::new()).await?;
//! Ok(())
//! # };
//! ```
#![cfg(feature = "ku-coin")]

mod endpoint;
mod integration_tests;

pub use endpoint::*;

use crate::{
  types::{MaxUrl, MaxUrlParts},
  utils::GenericTime,
};
use arrayvec::{ArrayString, ArrayVec};

#[derive(Debug)]
pub struct KuCoin;

#[cfg(feature = "tokio-tungstenite")]
impl KuCoin {
  pub async fn tokio_tungstenite(
    bullet_server: &V1BulletServerRes,
    token: &str,
  ) -> crate::Result<
    crate::Pair<
      Self,
      crate::CommonParams<crate::network::WsParams, ()>,
      crate::network::TokioTungstenite,
    >,
  > {
    use crate::{network::WsParams, CommonParams, Pair};
    use core::fmt::Write;
    use tokio_tungstenite::connect_async;

    let mut url = ArrayString::new();
    url.write_fmt(format_args!(
      "{endpoint}?token={token}&connectId={connect_id}&acceptUserMessage=true",
      endpoint = &*bullet_server.endpoint,
      connect_id = _timestamp()?
    ))?;
    Ok(Pair::new(
      connect_async(url.as_str()).await?.0,
      CommonParams::new(WsParams { _url_parts: crate::utils::UrlParts::from_url(url)? }, ()),
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

#[derive(Debug, serde::Deserialize)]
pub struct GenericDataResponse<T> {
  pub code: ArrayString<8>,
  pub data: T,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BulletServerRes {
  pub encrypt: bool,
  pub endpoint: MaxUrl,
  pub ping_interval: u64,
  pub ping_timeout: u64,
  pub protocol: ArrayString<12>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BulletRes {
  pub instance_servers: ArrayVec<V1BulletServerRes, 4>,
  pub token: ArrayString<200>,
}

pub(crate) fn _timestamp() -> crate::Result<i64> {
  Ok(GenericTime::now()?.timestamp()?.as_millis().try_into()?)
}
