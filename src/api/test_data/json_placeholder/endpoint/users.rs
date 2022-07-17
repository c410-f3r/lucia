use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  UsersReq<;;>

  |raw: Vec<UsersRes>| -> Vec<UsersRes> { Ok(raw) }

  UsersParams() -> crate::Result<()> {
    |cp| {
      cp._method = HttpMethod::_Get;
      cp._url_parts.set_path(format_args!("/users"))?;
    }
  }

  users() {
    || {
      UsersReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersRes {
  pub id: u32,
  pub name: ArrayString<24>,
  pub username: ArrayString<16>,
  pub email: ArrayString<25>,
  pub address: UsersAddressRes,
  pub phone: ArrayString<21>,
  pub website: ArrayString<14>,
  pub company: UsersCompanyRes,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersAddressRes {
  pub street: ArrayString<17>,
  pub suite: ArrayString<10>,
  pub city: ArrayString<14>,
  pub zipcode: ArrayString<11>,
  pub geo: UsersAddressGeoRes,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersAddressGeoRes {
  pub lat: ArrayString<9>,
  pub lng: ArrayString<9>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersCompanyRes {
  pub name: ArrayString<18>,
  pub catch_phrase: ArrayString<40>,
  pub bs: ArrayString<36>,
}
