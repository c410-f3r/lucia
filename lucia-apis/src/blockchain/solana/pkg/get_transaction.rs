#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SolanaHttpPackagesAux, TransactionEncoding, TransactionOutput,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct GetTransactionReqData<S>(
    #[pkg::field(name = "hash")] S,
    #[pkg::field(name = "config")] Option<GetTransactionConfigReqData>,
  )
  where
    S: AsRef<str>;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type GetTransactionResData = TransactionOutput;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetTransactionConfigReqData {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<TransactionEncoding>,
  }
}
