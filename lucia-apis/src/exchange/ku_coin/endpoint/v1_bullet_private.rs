use crate::exchange::ku_coin::{GenericDataResponse, KuCoin, V1BulletRes};
use alloc::boxed::Box;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = GenericDataResponse<Box<V1BulletRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|json_request;

  V1BulletPrivateReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1BulletPrivateParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Post;
      hp.tp.url_parts.set_path(format_args!("/api/v1/bullet-private"))?;
    }
  }

  v1_bullet_private() {
    || {
      V1BulletPrivateReq
    }
  }
}
