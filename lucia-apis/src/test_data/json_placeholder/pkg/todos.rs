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
    params.manage("todos", req_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type TodosGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct TodosReq;

  #[pkg::res_data]
  pub type TodosRes = GenericRes;

  /// Todo
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Todo {
    /// User id
    pub user_id: u32,
    /// Id
    pub id: u32,
    /// Title
    pub title: ArrayString<86>,
    /// Completed
    pub completed: bool,
  }
}
