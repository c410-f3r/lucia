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
  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetLatestBlockhashReq(
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "commitment")]
    Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "min_context_slot")]
    Option<u64>,
  );

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[pkg::res_data]
  pub type GetLatestBlockhashRes = JsonRpcResponseResultWithContext<GetLatestBlockhash>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetLatestBlockhash {
    /// Block Base58 identifier.
    #[serde(deserialize_with = "crate::misc::deserialize_array_from_base58")]
    pub blockhash: SolanaBlockhash,
    /// Last block height at which the blockhash will be valid
    pub last_valid_block_height: u64,
  }
}
