#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSignaturesForAddress")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SolanaHttpPkgsAux, SolanaSignatureHashStr, TransactionError,
  };
  use lucia::misc::AsyncTrait;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSignaturesForAddressReq<S>(
    #[pkg::field(name = "address")] S,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSignaturesForAddressConfig<S>>,
  )
  where
    S: AsyncTrait;

  #[pkg::res_data]
  pub type GetSignaturesForAddressRes = Vec<GetSignaturesForAddress>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignaturesForAddress {
    /// Base58 identifier
    pub signature: SolanaSignatureHashStr,
    /// The slot that contains the block with the transaction.
    pub slot: u64,
    /// Filled if unsuccessful.
    pub err: Option<TransactionError>,
    /// Memo associated with the transaction, null if no memo is present.
    pub memo: Option<String>,
    /// Estimated production time, as Unix timestamp
    pub block_time: Option<i64>,
    /// Commitment
    pub confirmation_status: Option<Commitment>,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignaturesForAddressConfig<S> {
    /// Maximum transaction signatures to return (between 1 and 1,000, default: 1,000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Starts searching backwards from this transaction signature. If not provided the search
    /// starts from the top of the highest max confirmed block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<S>,
    /// Searches until this transaction signature, if found before limit reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<S>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }
}
