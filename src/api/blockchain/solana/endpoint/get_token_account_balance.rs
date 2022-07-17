use crate::{
  api::blockchain::solana::{
    program::spl_token::AccountBalance, Commitment, JsonRpcResponseResultWithContext, Solana,
  },
  utils::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getTokenAccountBalance" => GetTokenAccountBalanceReq<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, Commitment>
  )

  |
    raw: JsonRpcResponseResultWithContext<AccountBalance>
  | -> JsonRpcResponseResultWithContext<AccountBalance> {
    raw
  }

  get_token_account_balance(account: S, commitment: Option<Commitment>) {
    GetTokenAccountBalanceReq(OneMandAndOneOpt(account, commitment))
  }
}
