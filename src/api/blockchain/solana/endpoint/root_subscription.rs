use crate::api::blockchain::solana::Solana;

_create_json_rpc_endpoint! {
  Solana;

  "rootSubscribe" => RootSubscribeReq<;;>

  |raw: u64| -> u64 { raw }

  root_subscribe(){ RootSubscribeReq }
}

_create_json_rpc_endpoint! {
  Solana;

  "rootUnsubscribe" => RootUnsubscribeReq<;;>([u64; 1])

  |raw: bool| -> bool { raw }

  root_unsubscribe(id: u64) { RootUnsubscribeReq([id]) }
}
