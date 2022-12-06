#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSignatureStatuses")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaHttpPkgsAux, TransactionError,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetSignatureStatusesReq<'signatures, S>(
    #[pkg::field(name = "signatures")] &'signatures [S],
    #[pkg::field(name = "config")] Option<GetSignatureStatusesConfig>,
  )
  where
    S: AsRef<str> + Send + Sync;

  #[pkg::res_data]
  pub type GetSignatureStatusesRes =
    JsonRpcResponseResultWithContext<Vec<Option<GetSignatureStatusesResElem>>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetSignatureStatusesConfig {
    search_transaction_history: bool,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetSignatureStatusesResElem {
    /// Commitment
    pub confirmation_status: Commitment,
    /// Number of blocks since signature confirmation.
    pub confirmations: Option<usize>,
    /// Filled if the transaction failed.
    pub err: Option<TransactionError>,
    /// The slot the transaction was processed
    pub slot: u64,
  }
}
