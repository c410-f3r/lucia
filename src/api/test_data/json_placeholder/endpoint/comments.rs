use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  CommentsReq<;;>

  |raw: Vec<CommentsRes>| -> Vec<CommentsRes> { Ok(raw) }

  comments() -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.try_push_str("/comments")?;
      CommentsReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsRes {
  pub post_id: u32,
  pub id: u32,
  pub name: ArrayString<81>,
  pub email: ArrayString<33>,
  pub body: String,
}
