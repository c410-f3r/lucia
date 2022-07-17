use crate::api::blockchain::solana::{
  deserialize_array_from_base58, Commitment, CommitmentMand, JsonRpcResponseResultWithContext,
  Solana, SolanaBlockhash,
};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getLatestBlockhash" => GetLatestBlockhashReq<;;>([Option<CommitmentMand>; 1])

  |raw: JsonRpcResponseResultWithContext<GetLatestBlockhashRes>| -> JsonRpcResponseResultWithContext<GetLatestBlockhashRes> { raw }

  get_latest_blockhash(opt: Option<Commitment>) {
    GetLatestBlockhashReq([opt.map(|elem| CommitmentMand { commitment: elem })])
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestBlockhashRes {
  #[serde(deserialize_with = "deserialize_array_from_base58")]
  pub blockhash: SolanaBlockhash,
  pub last_valid_block_height: u64,
}
