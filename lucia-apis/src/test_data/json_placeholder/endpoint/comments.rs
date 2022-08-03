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

  CommentsReq<;;>

  |raw: ResBox, _resp| -> ResBox { Ok(raw) }

  CommentsParams(
    method: Method,
    id_opt: Option<u32>,
    nested_opt: Option<&'reqp str>,
    query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)]
  ) -> crate::Result<()> {
    |cp| {
      params_management("comments", cp, method, id_opt, nested_opt, query)?;
    }
  }

  comments() {
    || {
      CommentsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CommentRes {
  pub post_id: u32,
  pub id: u32,
  pub name: ArrayString<81>,
  pub email: ArrayString<33>,
  pub body: String,
}
