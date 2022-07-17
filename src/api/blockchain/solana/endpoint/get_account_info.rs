use crate::{
  api::blockchain::solana::{
    Account, CommitmenOptDataSliceOptEncodingMand, JsonRpcResponseResultWithContext, Solana,
  },
  utils::{into_rslt, OneMandAndOneOpt},
};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getAccountInfo" => GetAccountInfoReq<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, CommitmenOptDataSliceOptEncodingMand>
  )

  |raw: JsonRpcResponseResultWithContext<Option<Account>>| -> JsonRpcResponseResultWithContext<crate::Result<Account>> {
    JsonRpcResponseResultWithContext {
      context: raw.context,
      value: into_rslt(raw.value)
    }
  }

  get_account_info(pubkey: S, opt: Option<CommitmenOptDataSliceOptEncodingMand>) {
    GetAccountInfoReq(OneMandAndOneOpt(pubkey, opt))
  }
}
