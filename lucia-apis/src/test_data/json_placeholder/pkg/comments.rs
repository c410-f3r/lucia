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
    params_management("comments", params, req_params)?;
    Ok(())
  }

  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub type CommentsGenericParams<'any> = GenericParams<'any>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct CommentsReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type CommentsResData = GenericResData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct CommentsElemResData {
    pub post_id: u32,
    pub id: u32,
    pub name: ArrayString<81>,
    pub email: ArrayString<33>,
    pub body: String,
  }
}
