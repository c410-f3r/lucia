#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_call")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockId, CallRequest, EthereumHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthCallReq<'any>(
    #[pkg::field(name = "block_id")] Option<&'any BlockId>,
    #[pkg::field(name = "cr")] &'any CallRequest,
  );

  #[pkg::res_data]
  pub type EthCallRes = Option<crate::blockchain::ethereum::Bytes>;
}
