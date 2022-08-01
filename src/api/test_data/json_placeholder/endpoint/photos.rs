use crate::{
  api::test_data::json_placeholder::{endpoint::params_management, JsonPlaceholder, ResBox},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
  utils::DebugDisplay,
};
use arrayvec::ArrayString;

_create_endpoint! {
  JsonPlaceholder => JsonResponse|JsonRequest|_json_request;

  PhotosReq<;;>

  |raw: ResBox, _resp| -> ResBox { Ok(raw) }

  PhotosParams(
    method: Method,
    id_opt: Option<u32>,
    nested_opt: Option<&'reqp str>,
    query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)]
  ) -> crate::Result<()> {
    |cp| {
      params_management("photos", cp, method, id_opt, nested_opt, query)?;
    }
  }

  photos() {
    || {
      PhotosReq
    }
  }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PhotoRes {
  pub album_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub url: ArrayString<38>,
  pub thumbnail_url: ArrayString<38>,
}
