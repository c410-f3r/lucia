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

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetBlockReq(
    #[pkg::field(name = "slot")] u64,
    #[pkg::field(name = "config")] Option<GetBlockConfig>,
  );

  #[pkg::res_data]
  pub type GetBlockRes = Option<Block>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetBlockConfig {
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// Transaction encoding.
    pub encoding: Option<TransactionEncoding>,
    /// Rewards
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rewards: Option<bool>,
    /// Transaction details
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub transaction_details: Option<TransactionDetails>,
  }
}
