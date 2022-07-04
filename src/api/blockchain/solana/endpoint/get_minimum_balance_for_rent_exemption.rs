use crate::{
  api::blockchain::solana::{endpoint::Commitment, Solana},
  utils::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getMinimumBalanceForRentExemption" => GetMinimumBalanceForRentExemptionReq<;;>(OneMandAndOneOpt<usize, Commitment>)

  |raw: u64| -> u64 { raw }

  get_minimum_balance_for_rent_exemption(len: usize, opt: Option<Commitment>) {
    GetMinimumBalanceForRentExemptionReq(OneMandAndOneOpt(len, opt))
  }
}
