#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_getLogs")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{EthereumHttpPkgsAux, Filter, Log};
  use alloc::vec::Vec;

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthGetLogsReq<'filter>(#[pkg::field(name = "filter")] &'filter Filter);

  #[pkg::res_data]
  pub type EthGetLogsRes = Option<Vec<Log>>;
}
