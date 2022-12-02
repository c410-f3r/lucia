#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSignatureStatuses")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaHttpPackagesAux, TransactionError,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetSignatureStatusesReqData<'signatures, S>(
    #[pkg::field(name = "signatures")] &'signatures [S],
    #[pkg::field(name = "config")] Option<GetSignatureStatusesConfig>,
  )
  where
    S: AsRef<str> + Send + Sync;

  #[pkg::res_data]
  pub type GetSignatureStatusesResData =
    JsonRpcResponseResultWithContext<Vec<Option<GetSignatureStatusesResElem>>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetSignatureStatusesConfig {
    search_transaction_history: bool,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetSignatureStatusesResElem {
    pub confirmation_status: Commitment,
    pub confirmations: Option<usize>,
    pub err: Option<TransactionError>,
    pub slot: u64,
  }
}
