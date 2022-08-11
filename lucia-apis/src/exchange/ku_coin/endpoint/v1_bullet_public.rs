use crate::{
  exchange::ku_coin::{GenericDataResponse, KuCoin},
  misc::_MaxUrl,
};
use alloc::boxed::Box;
use arrayvec::{ArrayString, ArrayVec};
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = GenericDataResponse<Box<V1BulletPublicRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|json_request;

  V1BulletPublicReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1BulletPublicParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Post;
      hp.tp.url_parts.set_path(format_args!("/api/v1/bullet-public"))?;
    }
  }

  v1_bullet_public() {
    || {
      V1BulletPublicReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1BulletPublicServersRes {
  pub encrypt: bool,
  pub endpoint: _MaxUrl,
  pub ping_interval: u64,
  pub ping_timeout: u64,
  pub protocol: ArrayString<12>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1BulletPublicRes {
  pub instance_servers: ArrayVec<V1BulletPublicServersRes, 4>,
  pub token: ArrayString<200>,
}
