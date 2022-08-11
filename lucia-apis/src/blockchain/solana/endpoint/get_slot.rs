use crate::blockchain::solana::{Commitment, Solana};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getSlot" => GetSlotReq<;;>(Option<Commitment>)

  |raw: Wrapper| -> u64 { raw.0 }

  get_slot(opt: Option<Commitment>) { GetSlotReq(opt) }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Wrapper(u64);
