#[lucia_macros::pkg(
  api(crate::test_data::json_placeholder::JsonPlaceholder),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{
    pkg::params_management, GenericParams, GenericResData, JsonPlaceholderHttpPackagesAux,
  };
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(
    params: &mut GenericParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    params_management("users", params, req_params)?;
    Ok(())
  }

  #[pkg::params]
  #[lucia_macros::pkg_doc]
  pub type UsersGenericParams<'any> = GenericParams<'any>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct UsersReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type UsersResData = GenericResData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct UsersAddressResData {
    pub street: ArrayString<17>,
    pub suite: ArrayString<10>,
    pub city: ArrayString<14>,
    pub zipcode: ArrayString<11>,
    pub geo: UsersGeoResData,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct UsersCompanyResData {
    pub name: ArrayString<18>,
    pub catch_phrase: ArrayString<40>,
    pub bs: ArrayString<36>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct UsersElemResData {
    pub id: u32,
    pub name: ArrayString<24>,
    pub username: ArrayString<16>,
    pub email: ArrayString<25>,
    pub address: UsersAddressResData,
    pub phone: ArrayString<21>,
    pub website: ArrayString<14>,
    pub company: UsersCompanyResData,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct UsersGeoResData {
    pub lat: ArrayString<9>,
    pub lng: ArrayString<9>,
  }
}
