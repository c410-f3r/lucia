#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getLatestBlockhash")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaBlockhash, SolanaHttpPkgsAux,
  };
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetLatestBlockhashReq(
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "commitment")]
    pub Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "min_context_slot")]
    pub Option<u64>,
  );

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[pkg::res_data]
  pub type GetLatestBlockhashRes = JsonRpcResponseResultWithContext<GetLatestBlockhashResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetLatestBlockhashResElem {
    /// Block Base58 identifier.
    #[cfg_attr(
      feature = "serde",
      serde(deserialize_with = "crate::misc::deserialize_array_from_base58")
    )]
    pub blockhash: SolanaBlockhash,
    /// Last block height at which the blockhash will be valid
    pub last_valid_block_height: u64,
  }
}
