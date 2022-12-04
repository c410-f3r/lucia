#[lucia_macros::pkg(
  api(crate::test_data::json_placeholder::JsonPlaceholder),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{
    pkg::params_management, GenericParams, GenericRes, JsonPlaceholderHttpPackagesAux,
  };
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    params_management("users", params, req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type UsersGenericParams<'any> = GenericParams<'any>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct UsersReq;

  #[pkg::res_data]
  pub type UsersRes = GenericRes;

  /// User
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct User {
    /// Id
    pub id: u32,
    /// Name
    pub name: ArrayString<24>,
    /// Username
    pub username: ArrayString<16>,
    /// Email
    pub email: ArrayString<25>,
    /// Address
    pub address: UserAddress,
    /// Phone
    pub phone: ArrayString<21>,
    /// Website
    pub website: ArrayString<14>,
    /// Company
    pub company: UserCompany,
  }

  /// User address
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct UserAddress {
    /// Street
    pub street: ArrayString<17>,
    /// Suite
    pub suite: ArrayString<10>,
    /// City
    pub city: ArrayString<14>,
    /// Zip-code
    pub zipcode: ArrayString<11>,
    /// User geographic parameters
    pub geo: UserGeoParams,
  }

  /// User company
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct UserCompany {
    /// Name
    pub name: ArrayString<18>,
    /// What the company does.
    pub catch_phrase: ArrayString<40>,
    /// Tags
    pub bs: ArrayString<36>,
  }

  /// User geographic parameters.
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct UserGeoParams {
    /// Latitude
    pub lat: ArrayString<9>,
    /// Longitude
    pub lng: ArrayString<9>,
  }
}
