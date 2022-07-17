use crate::{
  api::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, Solana, TransactionError,
  },
  utils::OneMandAndOneOpt,
};
use arrayvec::ArrayVec;
use cl_aux::ArrayWrapper;

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getSignatureStatuses" => GetSignatureStatusesReq<;const N: usize;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<
      ArrayWrapper<S, N>,
      GetSignatureStatusesCfg
    >
  )

  |
    raw: JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, N>>
  | -> JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, N>> {
    raw
  }

  get_signature_statuses(
    opt: Option<GetSignatureStatusesCfg>,
    transaction_addresses: ArrayWrapper<S, N>,
  ) {
    GetSignatureStatusesReq(OneMandAndOneOpt(
      transaction_addresses,
      opt,
    ))
  }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureStatusesCfg {
  pub search_transaction_history: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureStatusesRes {
  pub confirmation_status: Commitment,
  pub confirmations: Option<usize>,
  pub err: Option<TransactionError>,
  pub slot: u64,
}
