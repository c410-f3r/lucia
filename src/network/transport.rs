#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "surf")]
mod surf;
mod test;
#[cfg(feature = "tokio-tungstenite")]
mod tokio_tungstenite;
mod unit;

#[cfg(feature = "reqwest")]
pub use self::reqwest::*;
#[cfg(feature = "surf")]
pub use self::surf::*;
#[cfg(feature = "tokio-tungstenite")]
pub use self::tokio_tungstenite::*;
pub use test::*;

use crate::{
  utils::{decode_many, decode_one},
  Request, RequestManager, RequestParams, Requests,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

/// Any means of transferring data between two parties.
///
/// Internally in every method all custom/protocol related parameters are temporally used to
/// tell how the given request should be issued.
///
/// Please, see the [crate::Request] implementation of the desired request to know
/// more about the expected types as well as any other additional documentation.
///
/// # Example
///
/// The Covid-19 API provided by `M Media` has an endpoint called `cases` that accepts
/// different HTTP query parameters to modify returned data.
///
#[cfg_attr(feature = "covid-19", doc = "```rust,no_run")]
#[cfg_attr(not(feature = "covid-19"), doc = "```rust,ignore")]
/// # use lucia::{api::health::covid_19::CasesParams, network::{HttpParams, Transport}, Pair};
/// async fn request() -> lucia::Result<()> {
///   let (mut rm, mut trans) = Pair::new(
///     (),
///     HttpParams::from_origin("https://covid-api.mmediagroup.fr").unwrap()
///   )
///   .into_parts();
///   let req = rm.cases();
///   let _res = trans
///     .send_retrieve_and_decode_one(&mut rm, &req, CasesParams::new(Some("pt"), None, None))
///     .await?;
///   Ok(())
/// }
/// ```
///
/// As seen from above, [lucia::api::health::covid_19::CasesParams<'_>] comes from
/// <https://docs.rs/lucia/0.1.0/lucia/trait.Request.html#impl-Request-8>
#[async_trait::async_trait]
pub trait Transport<A, CP>
where
  A: Send,
  CP: Send,
{
  /// Sends a request without trying to retrieve any counterpart data.
  ///
  /// See the trait documentation for more information.
  async fn send<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<()>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send;

  /// Sends a request and then awaits its counterpart data response.
  ///
  /// See the trait documentation for more information.
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send;

  /// Convenient method used for JSON-RPC 2.0 batch requests
  ///
  /// See [crate::Requests] for more information.
  #[inline]
  async fn send_retrieve_and_decode_many<BUFFER, REQS, RPD>(
    &mut self,
    buffer: &mut BUFFER,
    rm: &mut RequestManager<A, CP>,
    reqs: &REQS,
    rpd: RPD,
  ) -> crate::Result<()>
  where
    BUFFER: Send,
    REQS: Debug + Requests<BUFFER, CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    let bytes = &if let (0, Some(0)) = reqs.size_hint() {
      Vec::new()
    } else {
      self.send_and_retrieve(rm, reqs, rpd).await?
    };
    decode_many(buffer, bytes, reqs)
  }

  /// Similar to [Self::send_retrieve_and_decode_many] but expects a single data return from the
  /// counterpart. Can be currently used by any protocol.
  ///
  /// See [crate::Request] for more information.
  #[inline]
  async fn send_retrieve_and_decode_one<REQ, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: &REQ,
    rpd: RPD,
  ) -> crate::Result<REQ::ProcessedResponse>
  where
    REQ: Debug + Request<CP, RPD> + Send + Serialize + Sync,
    REQ::RawResponse: Debug + for<'de> Deserialize<'de>,
    RPD: Send,
  {
    let bytes = self.send_and_retrieve(rm, req, rpd).await?;
    decode_one(&bytes, req)
  }
}

#[async_trait::async_trait]
impl<A, CP, U> Transport<A, CP> for &mut U
where
  A: Send,
  CP: Send,
  U: Send + Transport<A, CP>,
{
  #[inline]
  async fn send<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<()>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    (*self).send(rm, req, rpd).await
  }

  #[inline]
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    (*self).send_and_retrieve(rm, req, rpd).await
  }
}
