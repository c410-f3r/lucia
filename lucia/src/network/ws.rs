use crate::network::transport::TransportParams;

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
