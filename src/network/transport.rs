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
  dnsn::{Deserialize, Serialize},
  utils::{decode_many, decode_one},
  Request, RequestManager, RequestParamsModifier, Requests,
};
use alloc::{boxed::Box, vec::Vec};

/// Any means of transferring data between two parties.
///
/// Internally in every method all custom related parameters are temporally used to
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
/// ```rust-no_run
/// # #[cfg(feature = "m-media-covid-19")] async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   api::health::m_media_covid_19::CasesParams,
///   network::{http::ReqParams, Transport},
///   CommonParams, Pair, RequestManager,
/// };
/// let (mut rm, mut trans) = Pair::new(
///   RequestManager::new(
///     <_>::default(),
///     CommonParams::new((), ReqParams::from_origin("https://covid-api.mmediagroup.fr")?, ())
///   ),
///   (),
/// ).into_parts();
/// let req = rm.cases();
/// let _res = trans
///   .send_retrieve_and_decode_one(&mut rm, &req, CasesParams::new(Some("pt"), None, None))
///   .await?;
/// # Ok(()) }
/// ```
///
/// As seen from above, [lucia::api::health::covid_19::CasesParams<'_>] comes from
/// <https://docs.rs/lucia/0.1.0/lucia/trait.Request.html#impl-Request-8>
///
/// # Types
///
/// * `A`: **A**PI
/// * `CP`: **C**ommon **P**arameters
/// * `DRSR`: **D**eserialize**R**/**S**erialize**R**
#[async_trait::async_trait]
pub trait Transport<A, CP, DRSR>
where
  A: Send,
  CP: Send,
  DRSR: Send,
{
  /// Custom metadata attached to the transport. For example, HTTP has response headers as metadata.
  type Metadata;

  /// Sends a request without trying to retrieve any counterpart data.
  ///
  /// See the trait documentation for more information.
  async fn send<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<Self::Metadata>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send;

  /// Sends a request and then awaits its counterpart data response.
  ///
  /// See the trait documentation for more information.
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<(Self::Metadata, Vec<u8>)>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send;

  /// Convenient method used for JSON-RPC 2.0 batch requests
  ///
  /// See [crate::Requests] for more information.
  #[inline]
  async fn send_retrieve_and_decode_many<BUFFER, REQS, REQP>(
    &mut self,
    buffer: &mut BUFFER,
    rm: &mut RequestManager<A, CP, DRSR>,
    reqs: &REQS,
    reqp: REQP,
  ) -> crate::Result<()>
  where
    BUFFER: Send,
    REQS: Requests<BUFFER, CP, DRSR, REQP, Self::Metadata> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    let (resp, bytes) = self.send_and_retrieve(rm, reqs, reqp).await?;
    decode_many(buffer, &bytes, &mut rm.drsr, reqs, &resp)
  }

  /// Similar to [Self::send_retrieve_and_decode_many] but expects a single data return from the
  /// counterpart. Can be currently used by any data format.
  ///
  /// See [crate::Request] for more information.
  #[inline]
  async fn send_retrieve_and_decode_one<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<REQ::ProcessedResponse>
  where
    REQ: Request<CP, REQP, Self::Metadata> + Send + Serialize<DRSR> + Sync,
    REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
    REQP: Send,
  {
    let (resp, bytes) = self.send_and_retrieve(rm, req, reqp).await?;
    decode_one(&bytes, &mut rm.drsr, req, &resp)
  }
}

#[async_trait::async_trait]
impl<A, CP, DRSR, U> Transport<A, CP, DRSR> for &mut U
where
  A: Send,
  CP: Send,
  DRSR: Send,
  U: Send + Transport<A, CP, DRSR>,
{
  type Metadata = U::Metadata;

  #[inline]
  async fn send<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<Self::Metadata>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    (*self).send(rm, req, reqp).await
  }

  #[inline]
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<(Self::Metadata, Vec<u8>)>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    (*self).send_and_retrieve(rm, req, reqp).await
  }
}
