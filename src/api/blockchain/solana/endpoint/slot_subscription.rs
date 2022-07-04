use crate::api::blockchain::solana::Solana;

_create_json_rpc_endpoint! {
  Solana;

  "slotSubscribe" => SlotSubscribeReq<;;>

  |raw: u64| -> u64 { raw }

  slot_subscribe() { SlotSubscribeReq }
}

_create_json_rpc_endpoint! {
  Solana;

  "slotUnsubscribe" => SlotUnsubscribeReq<;;>([u64; 1])

  |raw: bool| -> bool { raw }

  slot_unsubscribe(id: u64) { SlotUnsubscribeReq([id]) }
}
