//! Utility functions and structures

pub(crate) mod seq_visitor;

mod byte_buffer;
mod debug_display;
mod from_bytes;
mod generic_time;
mod pair;
mod query_writer;
mod request_counter;
mod request_limit;
mod request_throttling;
mod url;

pub use byte_buffer::*;
pub use debug_display::*;
pub use from_bytes::*;
pub use generic_time::*;
pub use pair::*;
pub use request_counter::*;
pub use request_limit::*;
pub use request_throttling::*;
pub use url::*;

pub(crate) use query_writer::QueryWriter;

use crate::{
  network::transport::Transport,
  package::{Package, PackagesAux},
};
use core::time::Duration;

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

/// Used in all implementations of [crate::Transport::send] and/or
/// [crate::Transport::send_and_receive`].
#[inline]
pub(crate) fn log_req<DRSR, P, T>(
  _pgk: &mut P,
  _pkgs_aux: &mut PackagesAux<P::Api, DRSR, T::Params>,
  _trans: &T,
) where
  P: Package<DRSR, T::Params>,
  T: Transport<DRSR>,
{
  _debug!(trans_ty = display(_trans.ty()), "Request: {:?}", {
    use crate::dnsn::Serialize;
    let mut vec = alloc::vec::Vec::new();
    _pgk
      .ext_req_ctnt_mut()
      .to_bytes(&mut vec, &mut _pkgs_aux.drsr)
      .and_then(|_| Ok(alloc::string::String::from_utf8(vec)?))
  });
}

/// Used in [crate::Transport::send_retrieve_and_decode_contained] and all implementations of
/// [crate::Requests::decode_responses].
///
/// Not used in [crate::Transport::send_retrieve_and_decode_many] because
/// [crate::Requests::decode_responses] takes precedence.
#[inline]
pub(crate) fn log_res(_res: &[u8]) {
  _debug!("Response: {:?}", core::str::from_utf8(_res));
}
