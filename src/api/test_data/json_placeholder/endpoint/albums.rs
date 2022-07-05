use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  AlbumsReq<;;>

  |raw: Vec<AlbumsRes>| -> Vec<AlbumsRes> { Ok(raw) }

  albums() -> crate::Result<:> {
    |api, tp| {
      tp._http_params._set(HttpMethod::Get, None, &api.origin);
      tp._http_params._url.try_push_str("/albums")?;
      AlbumsReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumsRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<75>,
}
