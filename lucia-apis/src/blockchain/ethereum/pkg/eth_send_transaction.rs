#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_sendTransaction")),
  error(crate::Error),
  transport(http, stub)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{EthereumHttpPkgsAux, TransactionRequest};
  use ethereum_types::H256;

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthSendTransactionReq<'tr>(#[pkg::field(name = "tr")] [&'tr TransactionRequest; 1]);

  #[pkg::res_data]
  pub type EthSendTransactionRes = Option<H256>;
}
