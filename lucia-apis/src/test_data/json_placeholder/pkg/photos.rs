#[lucia_macros::pkg(
  api(crate::test_data::json_placeholder::JsonPlaceholder),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{
    pkg::params_management, GenericParams, GenericRes, JsonPlaceholderHttpPkgsAux,
  };
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    params_management("photos", params, req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type PhotosGenericParams<'any> = GenericParams<'any>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct PhotosReq;

  #[pkg::res_data]
  pub type PhotosRes = GenericRes;

  /// Photo
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct Photo {
    /// Album id.
    pub album_id: u32,
    /// Id.
    pub id: u32,
    /// Title
    pub title: ArrayString<86>,
    /// URL
    pub url: ArrayString<38>,
    /// Thumbnail URL
    pub thumbnail_url: ArrayString<38>,
  }
}
