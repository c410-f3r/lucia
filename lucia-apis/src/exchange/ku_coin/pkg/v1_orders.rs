#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::exchange::ku_coin::{GenericDataResponse, KuCoinHttpPackagesAux};
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/orders"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V1OrdersReqData {
    pub client_oid: String,
    pub side: String,
    pub symbol: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: String,
    pub remark: String,
    pub stp: String,
  }

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V1OrdersResData = GenericDataResponse<V1OrdersElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1OrdersElemResData {}
}
