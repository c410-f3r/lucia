use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  AlbumsReq<;;>

  |raw: Vec<AlbumsRes>| -> Vec<AlbumsRes> { Ok(raw) }

  AlbumsParams() -> crate::Result<()> {
    |cp| {
      cp._method = HttpMethod::_Get;
      cp._url_parts.set_path(format_args!("/albums"))?;
    }
  }

  albums(){
    || {
      AlbumsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumsRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<75>,
}
