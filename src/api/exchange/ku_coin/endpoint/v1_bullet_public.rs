use crate::{
  api::exchange::ku_coin::{endpoint::GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::MaxUrl,
};
use arrayvec::{ArrayString, ArrayVec};

type Res = GenericDataResponse<V1BulletPublicRes>;

_create_json_endpoint! {
  KuCoin;

  V1BulletPublicReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  v1_bullet_public() {
    |api, tp| {
      tp.http_params._set(HttpMethod::Post, api.urls.v1_bullet_public.url());
      V1BulletPublicReq
    }
  }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BulletPublicServersRes {
  pub encrypt: bool,
  pub endpoint: MaxUrl,
  pub ping_interval: u64,
  pub ping_timeout: u64,
  pub protocol: ArrayString<12>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BulletPublicRes {
  pub instance_servers: ArrayVec<V1BulletPublicServersRes, 4>,
  pub token: ArrayString<200>,
}
