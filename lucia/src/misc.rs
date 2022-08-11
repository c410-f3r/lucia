//! Utility functions and structures

pub(crate) mod seq_visitor;

mod buffer;
mod common_params;
mod debug_display;
mod from_bytes;
mod from_error_ty;
mod generic_time;
mod pair;
mod query_writer;
mod request_counter;
mod request_limit;
mod request_throttling;
mod url_parts;

pub use buffer::*;
pub use common_params::*;
pub use debug_display::*;
pub use from_bytes::*;
pub use from_error_ty::*;
pub use generic_time::*;
pub use pair::*;
pub use request_counter::*;
pub use request_limit::*;
pub use request_throttling::*;
pub use url_parts::*;

pub(crate) use query_writer::QueryWriter;

use crate::{
  dnsn::{Deserialize, Serialize},
  req_res::{Request, Requests},
};
use core::time::Duration;

/// Shortcut for the internal machinery that decodes many request responses
#[inline]
pub(crate) fn decode_many<BUFFER, CP, DRSR, REQS, REQP, RESP>(
  buffer: &mut BUFFER,
  bytes: &[u8],
  drsr: &mut DRSR,
  reqs: &REQS,
  resp: &RESP,
) -> Result<(), REQS::Error>
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
) -> Result<REQ::ProcessedResponse, REQ::Error>
where
  REQ: Request<CP, DRSR, REQP, RESP> + Serialize<DRSR>,
{
  log(drsr, req, bytes);
  let raw: REQ::RawResponse = REQ::RawResponse::from_bytes(bytes, drsr)?;
  REQ::process(raw, resp)
}

/// Sleeps for the specified amount of time.
///
/// Intended for asynchronous usage, i.e., won't block threads.
#[inline]
pub async fn sleep(duration: Duration) -> crate::Result<()> {
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
