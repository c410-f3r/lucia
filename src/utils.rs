//! Utility functions and structures

pub(crate) mod seq_visitor;

mod buffer;
mod debug_display;
mod from_bytes;
mod generic_time;
mod one_mand_and_one_opt;
mod query_writer;
mod request_counter;
mod request_limit;
mod request_throttling;
mod url_parts;

pub use buffer::*;
pub use debug_display::*;
pub use from_bytes::*;
pub use generic_time::*;
pub use one_mand_and_one_opt::*;
pub use request_counter::*;
pub use request_limit::*;
pub use request_throttling::*;
pub use url_parts::*;

pub(crate) use query_writer::QueryWriter;
pub(crate) use url_parts::UrlPartsArrayString;

use crate::{
  dnsn::{Deserialize, Serialize},
  Request, Requests,
};
use arrayvec::ArrayString;
use core::{any::type_name, time::Duration};

/// Useful when a request returns an optional field that needs to be unwrapped in a
/// [core::result::Result] context.
#[inline]
#[track_caller]
pub fn into_rslt<T>(opt: Option<T>) -> crate::Result<T> {
  opt.ok_or(crate::Error::NoInnerValue(type_name::<T>()))
}

/// Shortcut for the internal machinery that decodes many request responses
#[inline]
pub(crate) fn decode_many<BUFFER, CP, DRSR, REQS, REQP, RESP>(
  buffer: &mut BUFFER,
  bytes: &[u8],
  drsr: &mut DRSR,
  reqs: &REQS,
  resp: &RESP,
) -> crate::Result<()>
where
  REQS: Requests<BUFFER, CP, DRSR, REQP, RESP> + Serialize<DRSR>,
{
  log(drsr, reqs, bytes);
  reqs.manage_responses(buffer, bytes, drsr, resp)
}

/// Shortcut for the internal machinery that decode one request response
#[inline]
pub(crate) fn decode_one<CP, DRSR, REQ, REQP, RESP>(
  bytes: &[u8],
  drsr: &mut DRSR,
  req: &REQ,
  resp: &RESP,
) -> crate::Result<REQ::ProcessedResponse>
where
  REQ: Request<CP, REQP, RESP> + Serialize<DRSR>,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  log(drsr, req, bytes);
  let raw: REQ::RawResponse = REQ::RawResponse::from_bytes(bytes, drsr)?;
  REQ::process(raw, resp)
}

#[cfg(all(feature = "bs58", feature = "serde"))]
#[inline]
pub(crate) fn deserialize_array_from_base58<'de, D, const N: usize>(
  deserializer: D,
) -> Result<[u8; N], D::Error>
where
  D: serde::Deserializer<'de>,
{
  use serde::de::Error;
  let s: &str = serde::Deserialize::deserialize(deserializer)?;
  let mut array = [0; N];
  bs58::decode(s)
    .into(&mut array)
    .ok()
    .and_then(|len| {
      if len != N {
        return None;
      }
      Some(())
    })
    .ok_or_else(|| D::Error::custom("Could not deserialize base58 hash string"))?;
  Ok(array)
}

#[cfg(feature = "serde")]
#[inline]
pub(crate) fn _deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: serde::Deserializer<'de>,
  T: Default,
{
  use serde::Deserialize;
  serde::de::IgnoredAny::deserialize(deserializer).map(|_| T::default())
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
pub(crate) fn log<DRSR, T>(_ds: &mut DRSR, _req: &T, _res: &[u8])
where
  T: Serialize<DRSR>,
{
  #[cfg(not(feature = "tracing"))]
  {}
  #[cfg(feature = "tracing")]
  {
    _debug!("Request: {:?}", {
      let mut vec = alloc::vec::Vec::new();
      _req.to_bytes(&mut vec, _ds).and_then(|_| Ok(alloc::string::String::from_utf8(vec)?))
    });
    _debug!("Response: {:?}", core::str::from_utf8(_res));
  }
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
