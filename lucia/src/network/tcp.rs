use crate::{misc::UrlString, network::transport::TransportParams};

#[derive(Debug)]
#[doc = generic_trans_params_doc!()]
pub struct TcpParams(TcpReqParams, TcpResParams);

impl TcpParams {
  /// For example, from `127.0.0.1:8090`.
  #[inline]
  pub fn from_url(url: &str) -> crate::Result<Self> {
    Ok(Self(TcpReqParams { url: UrlString::from_url(url.into())? }, TcpResParams))
  }
}

impl TransportParams for TcpParams {
  type ExternalRequestParams = TcpReqParams;
  type ExternalResponseParams = TcpResParams;

  #[inline]
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams) {
    (self.0, self.1)
  }
}

#[derive(Debug)]
#[doc = generic_trans_req_params_doc!("TCP")]
pub struct TcpReqParams {
  /// Used every time a send-like function is called.
  pub url: UrlString,
}

#[derive(Debug)]
#[doc = generic_trans_res_params_doc!("TCP")]
pub struct TcpResParams;
