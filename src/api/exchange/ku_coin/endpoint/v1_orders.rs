use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  network::HttpMethod,
};
use alloc::string::String;

type Res = GenericDataResponse<V1OrdersRes>;

_create_json_endpoint! {
  KuCoin;

  V1OrdersReq<;;>(V1OrdersReqParams)

  |raw: Res| -> Res { Ok(raw) }

  V1OrdersParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v1/orders"))?;
    }
  }

  v1_orders(params: V1OrdersReqParams) {
    || {
      V1OrdersReq(params)
    }
  }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrdersReqParams {
  client_oid: String,
  side: String,
  symbol: String,
  #[serde(rename = "type")]
  ty: String,
  remark: String,
  stp: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrdersRes {}
