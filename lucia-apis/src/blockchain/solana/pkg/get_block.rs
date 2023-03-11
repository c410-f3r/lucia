#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBlock")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Block, Commitment, SolanaHttpPkgsAux, TransactionDetails, TransactionEncoding,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockReq(
    #[pkg::field(name = "slot")] u64,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetBlockConfig>,
  );

  #[pkg::res_data]
  pub type GetBlockRes = Option<Block>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Transaction encoding.
    pub encoding: Option<TransactionEncoding>,
    /// Maxixmum transaction version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
    /// Rewards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<bool>,
    /// Transaction details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetails>,
  }
}
