use crate::test_data::json_placeholder::{endpoint::params_management, JsonPlaceholder, ResBox};
use alloc::string::String;
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  misc::DebugDisplay,
  network::http::Method,
};

_create_endpoint! {
  JsonPlaceholder => JsonResponse|JsonRequest|json_request;

  PostsReq<;;>

  |raw: ResBox, _resp| -> ResBox { Ok(raw) }

  PostsParams(
    method: Method,
    id_opt: Option<u32>,
    nested_opt: Option<&'reqp str>,
    query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)]
  ) -> crate::Result<()> {
    |cp| {
      params_management("posts", cp, method, id_opt, nested_opt, query)?;
    }
  }

  posts() {
    || {
      PostsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct PostRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub body: String,
}
