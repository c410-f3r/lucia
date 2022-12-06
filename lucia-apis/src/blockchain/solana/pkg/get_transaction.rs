#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SolanaHttpPkgsAux, TransactionEncoding, TransactionOutput,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetTransactionReq<S>(
    #[pkg::field(name = "hash")] S,
    #[pkg::field(name = "config")] Option<GetTransactionConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetTransactionRes = TransactionOutput;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetTransactionConfig {
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Transaction encoding
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<TransactionEncoding>,
  }
}
