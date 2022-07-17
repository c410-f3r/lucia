//! Utility functions and structures

mod generic_time;
mod one_mand_and_one_opt;
mod query_writer;
mod request_counter;
mod request_limit;
mod request_throttling;
mod seq_visitor;
mod url_parts;

use arrayvec::ArrayString;
pub use generic_time::GenericTime;
pub use one_mand_and_one_opt::OneMandAndOneOpt;
pub use request_counter::RequestCounter;
pub use request_limit::RequestLimit;
pub use request_throttling::RequestThrottling;
pub use url_parts::UrlParts;

#[allow(unused_imports)]
pub(crate) use query_writer::QueryWriter;
pub(crate) use seq_visitor::SeqVisitor;
pub(crate) use url_parts::UrlPartsArrayString;

use crate::{
  protocol::{JsonRpcNotification, JsonRpcResponse},
  Request, Requests,
};
use core::{any::type_name, fmt::Debug, time::Duration};
use serde::{de::IgnoredAny, Deserialize, Deserializer, Serialize};

/// Shortcut of `serde_json::from_slice::<JsonRpcNotification<_>>(bytes))`.
#[inline]
pub fn decode_json_rpc_notification<R>(bytes: &[u8]) -> crate::Result<JsonRpcNotification<R>>
where
  R: for<'de> Deserialize<'de>,
{
  Ok(serde_json::from_slice(bytes)?)
}

/// Useful when a request returns an optional field that needs to be unwrapped in a
/// [core::result::Result] context.
#[inline]
#[track_caller]
pub fn into_rslt<T>(opt: Option<T>) -> crate::Result<T> {
  opt.ok_or(crate::Error::NoInnerValue(type_name::<T>()))
}

/// Shortcut for the internal machinery that decodes many request responses
#[inline]
pub(crate) fn decode_many<BUFFER, CP, REQS, RPD>(
  buffer: &mut BUFFER,
  bytes: &[u8],
  reqs: &REQS,
) -> crate::Result<()>
where
  REQS: Debug + Requests<BUFFER, CP, RPD> + Serialize,
{
  log(reqs, bytes);
  reqs.manage_responses(buffer, bytes)
}

/// Shortcut for the internal machinery that decode one request response
#[inline]
pub(crate) fn decode_one<CP, REQ, RPD>(
  bytes: &[u8],
  req: &REQ,
) -> crate::Result<REQ::ProcessedResponse>
where
  REQ: Debug + Request<CP, RPD> + Serialize,
  REQ::RawResponse: Debug + for<'de> Deserialize<'de>,
{
  log(req, bytes);
  let res: REQ::RawResponse = serde_json::from_slice(bytes)?;
  REQ::process_response(res)
}

#[inline]
pub(crate) fn _deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Default,
{
  IgnoredAny::deserialize(deserializer).map(|_| T::default())
}

#[cfg(feature = "ku-coin")]
#[inline]
pub(crate) fn _encode_to_base64<const N: usize>(bytes: &[u8]) -> crate::Result<ArrayString<N>> {
  let mut buffer = [0; N];
  let len = base64::encode_config_slice(bytes, base64::STANDARD, &mut buffer);
  let mut s = ArrayString::from_byte_string(&buffer)?;
  s.truncate(len);
  Ok(s)
}

#[inline]
pub(crate) fn _i64_to_array_string(timestamp: i64) -> crate::Result<ArrayString<19>> {
  Ok(ArrayString::try_from(format_args!("{}", timestamp))?)
}

#[cfg(test)]
#[inline]
pub(crate) fn _init_tracing() {
  use tracing_subscriber::{
    fmt::{format::FmtSpan, Subscriber},
    util::SubscriberInitExt,
    EnvFilter,
  };
  let _ = Subscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
    .finish()
    .try_init();
}

#[inline]
pub(crate) fn log<T>(_req: &T, _res: &[u8])
where
  T: Serialize,
{
  #[cfg(not(feature = "tracing"))]
  {}
  #[cfg(feature = "tracing")]
  {
    _debug!("Request: {:?}", {
      use alloc::{borrow::ToOwned, vec::Vec};
      use serde::de::Error as _;
      let map = |el: &str| el.to_owned();
      let map_err = serde_json::Error::custom;
      let and_then = |el: Vec<u8>| core::str::from_utf8(&el).map(map).map_err(map_err);
      serde_json::to_vec(&_req).and_then(and_then)
    });
    _debug!("Response: {:?}", core::str::from_utf8(_res));
  }
}

#[inline]
pub(crate) fn manage_json_rpc_response<CP, REQ, RES, RPD>(
  req: &REQ,
  res: &JsonRpcResponse<RES>,
) -> crate::Result<()>
where
  REQ: Debug + Request<CP, RPD>,
{
  let id = req.id();
  if id != res.id {
    return Err(crate::Error::JsonRpcSentIdDiffersFromReceviedId(id, res.id));
  }
  Ok(())
}

pub(crate) async fn _sleep(duration: Duration) -> crate::Result<()> {
  #[cfg(all(feature = "async-std", not(feature = "tokio")))]
  {
    async_std::task::sleep(duration).await;
    Ok(())
  }
  #[cfg(all(feature = "tokio", not(feature = "async-std")))]
  {
    tokio::time::sleep(duration).await;
    Ok(())
  }
  #[cfg(any(
    all(feature = "async-std", feature = "tokio"),
    all(not(feature = "tokio"), not(feature = "async-std"))
  ))]
  {
    // Not great but still better than `std::thread::sleep`
    let now = GenericTime::now()?;
    loop {
      if now.elapsed()? >= duration {
        return Ok(());
      }
    }
  }
}
