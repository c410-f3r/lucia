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
  use lucia::misc::AsyncBounds;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSignatureStatusesReq<S>(
    #[pkg::field(name = "signatures")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSignatureStatusesConfig>,
  )
  where
    S: AsyncBounds;

  #[pkg::res_data]
  pub type GetSignatureStatusesRes =
    JsonRpcResponseResultWithContext<Vec<Option<GetSignatureStatuses>>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignatureStatusesConfig {
    search_transaction_history: bool,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignatureStatuses {
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
