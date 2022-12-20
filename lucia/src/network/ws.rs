use crate::network::transport::TransportParams;
use alloc::vec::Vec;

/// How the WebSocket request should be issued.
#[derive(Clone, Copy, Debug)]
pub enum WsReqParamsTy {
  /// As opaque bytes.
  Bytes,
  /// As a string.
  String,
}

#[derive(Debug)]
#[doc = generic_trans_params_doc!()]
pub struct WsParams(WsReqParams, WsResParams);

impl TransportParams for WsParams {
  type ExternalRequestParams = WsReqParams;
  type ExternalResponseParams = WsResParams;

  #[inline]
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams) {
    (self.0, self.1)
  }
}

impl Default for WsParams {
  #[inline]
  fn default() -> Self {
    Self(WsReqParams { ty: WsReqParamsTy::Bytes }, WsResParams)
  }
}

#[derive(Debug)]
#[doc = generic_trans_req_params_doc!("WebSocket")]
pub struct WsReqParams {
  /// Type
  pub ty: WsReqParamsTy,
}

impl WsReqParams {
  /// Clears modified parameters.
  #[inline]
  pub fn clear(&mut self) {
    self.ty = WsReqParamsTy::Bytes;
  }
}

#[derive(Debug)]
#[doc = generic_trans_res_params_doc!("WebSocket")]
pub struct WsResParams;

/// Abstracts all WebSocket implementations.
pub trait WebSocket
where
  Self: Sized,
{
  /// Different from other protocols, WebSockets only need an initial URL in order to create
  /// a channel or connection.
  async fn from_url(url: &str) -> crate::Result<Self>;

  /// Polls one single message and stores the result in the passed `bytes`.
  async fn receive_with_buffer(&mut self, bytes: &mut Vec<u8>) -> crate::Result<usize>;
}
