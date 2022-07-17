use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  PhotosReq<;;>

  |raw: Vec<PhotosRes>| -> Vec<PhotosRes> { Ok(raw) }

  PhotosParams() -> crate::Result<()> {
    |cp| {
      cp._method = HttpMethod::_Get;
      cp._url_parts.set_path(format_args!("/photos"))?;
    }
  }

  photos() {
    || {
      PhotosReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotosRes {
  pub album_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub url: ArrayString<38>,
  pub thumbnail_url: ArrayString<38>,
}
