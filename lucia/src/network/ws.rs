use crate::network::transport::TransportParams;

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
    Self(WsReqParams, WsResParams)
  }
}

#[derive(Debug)]
#[doc = generic_trans_req_params_doc!("WebSocket")]
pub struct WsReqParams;

#[derive(Debug)]
#[doc = generic_trans_res_params_doc!("WebSocket")]
pub struct WsResParams;
