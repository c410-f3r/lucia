use crate::{
  api::blockchain::solana::{
    CommitmentOptEncodingOpt, Solana, TransactionEncoding, TransactionOutput,
  },
  utils::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getTransaction" => GetTransactionReq<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, CommitmentOptEncodingOpt<TransactionEncoding>>
  )

  |raw: TransactionOutput| -> TransactionOutput { raw }

  get_transaction(
    hash: S,
    opt: Option<CommitmentOptEncodingOpt<TransactionEncoding>>,
  ) {
    GetTransactionReq(OneMandAndOneOpt(hash, opt))
  }
}
