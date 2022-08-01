use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin, V1BulletRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::boxed::Box;

type Res = GenericDataResponse<Box<V1BulletRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|_json_request;

  V1BulletPrivateReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1BulletPrivateParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Post;
      hp.tp._url_parts.set_path(format_args!("/api/v1/bullet-private"))?;
    }
  }

  v1_bullet_private() {
    || {
      V1BulletPrivateReq
    }
  }
}
