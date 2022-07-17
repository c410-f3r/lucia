use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  PostsReq<;;>

  |raw: Vec<PostsRes>| -> Vec<PostsRes> { Ok(raw) }

  PostsParams() -> crate::Result<()> {
    |cp| {
      cp._method = HttpMethod::_Get;
      cp._url_parts.set_path(format_args!("/posts"))?;
    }
  }

  posts() {
    || {
      PostsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub body: String,
}
