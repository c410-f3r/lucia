use crate::api::blockchain::solana::{
  Commitment, CommitmentMand, JsonRpcResponseResultWithContext, Solana, SolanaBlockhash,
};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getLatestBlockhash" => GetLatestBlockhashReq<;;>([Option<CommitmentMand>; 1])

  |raw: JsonRpcResponseResultWithContext<GetLatestBlockhashRes>| -> JsonRpcResponseResultWithContext<GetLatestBlockhashRes> { raw }

  get_latest_blockhash(opt: Option<Commitment>) {
    GetLatestBlockhashReq([opt.map(|elem| CommitmentMand { commitment: elem })])
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct GetLatestBlockhashRes {
  #[cfg_attr(
    feature = "serde",
    serde(deserialize_with = "crate::utils::deserialize_array_from_base58")
  )]
  pub blockhash: SolanaBlockhash,
  pub last_valid_block_height: u64,
}
