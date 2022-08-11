use crate::blockchain::solana::{Commitment, Solana};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getBlockHeight" => GetBlockHeightReq<;;>(GetBlockHeightReqParams)

  |raw: Wrapper| -> u64 {
    raw.0
  }

  get_block_height(commitment: Option<Commitment>, min_context_slot: Option<u64>) {
    GetBlockHeightReq(GetBlockHeightReqParams(
      commitment,
      min_context_slot
    ))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetBlockHeightReqParams(
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))] Option<Commitment>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))] Option<u64>,
);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Wrapper(u64);
