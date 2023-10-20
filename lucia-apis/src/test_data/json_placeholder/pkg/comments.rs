#[lucia_macros::pkg(
  api(crate::test_data::json_placeholder::JsonPlaceholder),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{GenericParams, GenericRes, JsonPlaceholderHttpPkgsAux};
  use alloc::string::String;
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    params.manage("comments", req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type CommentsGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CommentsReq;

  #[pkg::res_data]
  pub type CommentsRes = GenericRes;

  /// Comment
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
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
