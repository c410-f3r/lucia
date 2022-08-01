use crate::{
  api::test_data::json_placeholder::{endpoint::params_management, JsonPlaceholder, ResBox},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
  utils::DebugDisplay,
};
use arrayvec::ArrayString;

_create_endpoint! {
  JsonPlaceholder => JsonResponse|JsonRequest|_json_request;

  UsersReq<;;>

  |raw: ResBox, _resp| -> ResBox { Ok(raw) }

  UsersParams(
    method: Method,
    id_opt: Option<u32>,
    nested_opt: Option<&'reqp str>,
    query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)]
  ) -> crate::Result<()> {
    |cp| {
      params_management("users", cp, method, id_opt, nested_opt, query)?;
    }
  }

  users() {
    || {
      UsersReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct UserRes {
  pub id: u32,
  pub name: ArrayString<24>,
  pub username: ArrayString<16>,
  pub email: ArrayString<25>,
  pub address: UsersAddressRes,
  pub phone: ArrayString<21>,
  pub website: ArrayString<14>,
  pub company: UsersCompanyRes,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct UsersAddressRes {
  pub street: ArrayString<17>,
  pub suite: ArrayString<10>,
  pub city: ArrayString<14>,
  pub zipcode: ArrayString<11>,
  pub geo: UsersAddressGeoRes,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct UsersAddressGeoRes {
  pub lat: ArrayString<9>,
  pub lng: ArrayString<9>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct UsersCompanyRes {
  pub name: ArrayString<18>,
  pub catch_phrase: ArrayString<40>,
  pub bs: ArrayString<36>,
}
