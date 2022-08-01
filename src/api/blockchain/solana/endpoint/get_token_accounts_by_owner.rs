use crate::api::blockchain::solana::{
  Account, CommitmenOptDataSliceOptEncodingMand, JsonRpcRequestParamsWithTwoMandAndOneOpt,
  JsonRpcResponseResultWithContext, MintOrProgramId, Solana, SolanaAddressHashStr,
};
use arrayvec::ArrayVec;

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getTokenAccountsByOwner" => GetTokenAccountsByOwnerReq<;const N: usize;S AsRef<str> = &'static str>(
    JsonRpcRequestParamsWithTwoMandAndOneOpt<
    S,
    MintOrProgramId<S>,
    CommitmenOptDataSliceOptEncodingMand,
    >
  )

  |
    raw: JsonRpcResponseResultWithContext<ArrayVec<GetTokenAccountsByOwnerRes, N>>
  | -> JsonRpcResponseResultWithContext<ArrayVec<GetTokenAccountsByOwnerRes, N>> {
    raw
  }

  get_token_accounts_by_owner(
    account: S,
    mopi: MintOrProgramId<S>,
    opt: Option<CommitmenOptDataSliceOptEncodingMand>,
  ) {
    GetTokenAccountsByOwnerReq(JsonRpcRequestParamsWithTwoMandAndOneOpt(account, mopi, opt))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct GetTokenAccountsByOwnerRes {
  pub account: Account,
  pub pubkey: SolanaAddressHashStr,
}
