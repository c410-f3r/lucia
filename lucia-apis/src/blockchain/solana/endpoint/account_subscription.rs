use crate::{
  blockchain::solana::{CommitmentOptEncoding, Solana},
  misc::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  "accountSubscribe" => AccountSubscribeReq<;;S AsRef<str> = &'static str>(OneMandAndOneOpt<S, CommitmentOptEncoding>)

  |raw: u64| -> u64 { raw }

  account_subscribe(pubkey: S, opt: Option<CommitmentOptEncoding>) {
    AccountSubscribeReq(OneMandAndOneOpt(pubkey, opt))
  }
}

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "accountUnsubscribe" => AccountUnsubscribeReq<;;>([u64; 1])

  |raw: bool| -> bool { raw }

  account_unsubscribe(id: u64) { AccountUnsubscribeReq([id]) }
}
