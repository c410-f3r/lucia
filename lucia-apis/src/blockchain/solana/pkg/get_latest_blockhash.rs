#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getLatestBlockhash")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaBlockhash, SolanaHttpPackagesAux,
  };
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct GetLatestBlockhashReqData(
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "commitment")]
    pub Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "min_context_slot")]
    pub Option<u64>,
  );

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type GetLatestBlockhashResData = JsonRpcResponseResultWithContext<GetLatestBlockhashResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetLatestBlockhashResElem {
    #[cfg_attr(
      feature = "serde",
      serde(deserialize_with = "crate::misc::deserialize_array_from_base58")
    )]
    pub blockhash: SolanaBlockhash,
    pub last_valid_block_height: u64,
  }
}