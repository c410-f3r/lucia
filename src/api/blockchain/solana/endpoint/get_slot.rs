use crate::api::blockchain::solana::{endpoint::Commitment, Solana};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getSlot" => GetSlotReq<;;>(Option<Commitment>)

  |raw: u64| -> u64 { raw }

  get_slot(opt: Option<Commitment>) { GetSlotReq(opt) }
}
