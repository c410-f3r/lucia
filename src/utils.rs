#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
mod managed_request_resource;
mod one_mand_and_one_opt;
mod query_param_writer;
#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
mod request_counter;
mod request_limit;
#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
mod request_throttling;
mod seq_visitor;
mod url_parts;

#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
pub use managed_request_resource::*;
pub use one_mand_and_one_opt::*;
#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
pub use request_counter::*;
pub use request_limit::*;
#[cfg(any(all(feature = "async-std", feature = "std"), feature = "tokio"))]
pub use request_throttling::*;
pub use url_parts::*;

#[allow(
  // Used by some APIs
  unused_imports
)]
pub(crate) use query_param_writer::QueryParamWriter;
pub(crate) use seq_visitor::SeqVisitor;

use crate::{
  protocol::{JsonRpcNotification, JsonRpcResponse},
  Request, Requests,
};
use core::{any::type_name, fmt::Debug};
use serde::{de::IgnoredAny, Deserialize, Deserializer, Serialize};

#[inline]
pub fn decode_json_rpc_notification<R>(bytes: &[u8]) -> crate::Result<JsonRpcNotification<R>>
where
  R: for<'de> Deserialize<'de>,
{
  Ok(serde_json::from_slice(bytes)?)
}

#[inline]
pub fn decode_many<BUFFER, REQS>(
  buffer: &mut BUFFER,
  bytes: &[u8],
  reqs: &REQS,
) -> crate::Result<REQS::Output>
where
  REQS: Debug + Requests<BUFFER> + Serialize,
{
  log(reqs, bytes);
  reqs.manage_responses(buffer, bytes)
}

#[inline]
pub fn decode_one<REQ>(bytes: &[u8], req: &REQ) -> crate::Result<REQ::ProcessedResponse>
where
  REQ: Debug + Request + Serialize,
  REQ::RawResponse: Debug + for<'de> Deserialize<'de>,
{
  log(req, bytes);
  let res: REQ::RawResponse = serde_json::from_slice(bytes)?;
  REQ::process_response(res)
}

#[inline]
#[track_caller]
pub fn into_rslt<T>(opt: Option<T>) -> crate::Result<T> {
  opt.ok_or(crate::Error::NoInnerValue(type_name::<T>()))
}

#[inline]
pub(crate) fn _deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Default,
{
  IgnoredAny::deserialize(deserializer).map(|_| T::default())
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
pub(crate) fn log<T>(_req: T, _res: &[u8])
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
pub(crate) fn manage_json_rpc_response<REQ, RES>(
  req: &REQ,
  res: &JsonRpcResponse<RES>,
) -> crate::Result<()>
where
  REQ: Debug + Request,
{
  let id = req.id();
  if id != res.id {
    return Err(crate::Error::JsonRpcSentIdDiffersFromReceviedId(id, res.id));
  }
  Ok(())
}

#[cfg(any(feature = "tokio", feature = "async-std"))]
pub(crate) async fn _sleep(duration: core::time::Duration) {
  #[cfg(all(feature = "async-std", not(feature = "tokio")))]
  async_std::task::sleep(duration).await;
  #[cfg(all(feature = "tokio", not(feature = "async-std")))]
  tokio::time::sleep(duration).await;
  #[cfg(all(feature = "async-std", feature = "tokio"))]
  tokio::time::sleep(duration).await;
}
