use crate::api::blockchain::solana::Solana;

_create_json_rpc_endpoint! {
  Solana;

  "slotSubscribe" => SlotSubscribeReq<;;>

  |raw: WrapperU64| -> u64 { raw.0 }

  slot_subscribe() { SlotSubscribeReq }
}

_create_json_rpc_endpoint! {
  Solana;

  "slotUnsubscribe" => SlotUnsubscribeReq<;;>([u64; 1])

  |raw: WrapperBool| -> bool { raw.0 }

  slot_unsubscribe(id: u64) { SlotUnsubscribeReq([id]) }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct WrapperBool(bool);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct WrapperU64(u64);
