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
    params_management("comments", params, req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type CommentsGenericParams<'any> = GenericParams<'any>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct CommentsReq;

  #[pkg::res_data]
  pub type CommentsRes = GenericRes;

  /// Comment
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct Comment {
    /// Post id
    pub post_id: u32,
    /// Id
    pub id: u32,
    /// Name
    pub name: ArrayString<81>,
    /// Email
    pub email: ArrayString<33>,
    /// Body
    pub body: String,
  }
}
