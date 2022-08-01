use crate::api::blockchain::solana::Solana;

_create_json_rpc_endpoint! {
  Solana;

  "rootSubscribe" => RootSubscribeReq<;;>

  |raw: WrapperU64| -> u64 { raw.0 }

  root_subscribe(){ RootSubscribeReq }
}

_create_json_rpc_endpoint! {
  Solana;

  "rootUnsubscribe" => RootUnsubscribeReq<;;>([u64; 1])

  |raw: WrapperBool| -> bool { raw.0 }

  root_unsubscribe(id: u64) { RootUnsubscribeReq([id]) }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct WrapperBool(bool);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct WrapperU64(u64);
