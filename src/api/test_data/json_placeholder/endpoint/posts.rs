use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  PostsReq<;;>

  |raw: Vec<PostsRes>| -> Vec<PostsRes> { Ok(raw) }

  posts() -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.try_push_str("/posts")?;
      PostsReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub body: String,
}
