use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::string::String;

type Res = GenericDataResponse<V1OrdersRes>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|_json_request;

  V1OrdersReq<;;>(V1OrdersReqParams)

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1OrdersParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/orders"))?;
    }
  }

  v1_orders(params: V1OrdersReqParams) {
    || {
      V1OrdersReq(params)
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1OrdersReqParams {
  pub client_oid: String,
  pub side: String,
  pub symbol: String,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: String,
  pub remark: String,
  pub stp: String,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1OrdersRes {}
