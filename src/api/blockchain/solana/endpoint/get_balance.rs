use crate::{
  api::blockchain::solana::{Commitment, JsonRpcResponseResultWithContext, Solana},
  utils::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getBalance" => GetBalanceReq<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, Commitment>
  )

  |raw: JsonRpcResponseResultWithContext<u64>| -> JsonRpcResponseResultWithContext<u64> { raw }

  get_balance(account: S, commitment: Option<Commitment>) {
    GetBalanceReq(OneMandAndOneOpt(account, commitment))
  }
}
