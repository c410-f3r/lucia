use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin, V1BulletRes},
  network::HttpMethod,
};

type Res = GenericDataResponse<V1BulletRes>;

_create_json_endpoint! {
  KuCoin;

  V1BulletPrivateReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V1BulletPrivateParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Post;
      hp._url_parts.set_path(format_args!("/api/v1/bullet-private"))?;
    }
  }

  v1_bullet_private() {
    || {
      V1BulletPrivateReq
    }
  }
}
