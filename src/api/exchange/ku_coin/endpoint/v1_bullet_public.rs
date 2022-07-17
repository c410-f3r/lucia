use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::MaxUrl,
};
use arrayvec::{ArrayString, ArrayVec};

type Res = GenericDataResponse<V1BulletPublicRes>;

_create_json_endpoint! {
  KuCoin;

  V1BulletPublicReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V1BulletPublicParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Post;
      hp._url_parts.set_path(format_args!("/api/v1/bullet-public"))?;
    }
  }

  v1_bullet_public() {
    || {
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
