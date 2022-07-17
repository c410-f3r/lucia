use crate::api::blockchain::solana::{Commitment, Solana};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getBlockHeight" => GetBlockHeightReq<;;>(GetBlockHeightReqParams)

  |raw: u64| -> u64 {
    raw
  }

  get_block_height(commitment: Option<Commitment>, min_context_slot: Option<u64>) {
    GetBlockHeightReq(GetBlockHeightReqParams(
      commitment,
      min_context_slot
    ))
  }
}

#[derive(Debug, serde::Serialize)]
pub struct GetBlockHeightReqParams(
  #[serde(skip_serializing_if = "Option::is_none")] Option<Commitment>,
  #[serde(skip_serializing_if = "Option::is_none")] Option<u64>,
);
