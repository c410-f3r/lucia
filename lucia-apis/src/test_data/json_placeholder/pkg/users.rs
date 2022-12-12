#[lucia_macros::pkg(
  api(crate::test_data::json_placeholder::JsonPlaceholder),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{GenericParams, GenericRes, JsonPlaceholderHttpPkgsAux};
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    params.manage("users", req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type UsersGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct UsersReq;

  #[pkg::res_data]
  pub type UsersRes = GenericRes;

  /// User
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
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
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
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
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct UserCompany {
    /// Name
    pub name: ArrayString<18>,
    /// What the company does.
    pub catch_phrase: ArrayString<40>,
    /// Tags
    pub bs: ArrayString<36>,
  }

  /// User geographic parameters.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct UserGeoParams {
    /// Latitude
    pub lat: ArrayString<9>,
    /// Longitude
    pub lng: ArrayString<9>,
  }
}
