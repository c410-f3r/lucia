use crate::{
  blockchain::solana::{Commitment, CommitmentMand, Solana},
  misc::OneMandAndOneOpt,
};

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getMinimumBalanceForRentExemption" => GetMinimumBalanceForRentExemptionReq<;;>(OneMandAndOneOpt<usize, CommitmentMand>)

  |raw: Wrapper| -> u64 { raw.0 }

  get_minimum_balance_for_rent_exemption(len: usize, commitment: Option<Commitment>) {
    GetMinimumBalanceForRentExemptionReq(OneMandAndOneOpt(len, commitment.map(|elem| CommitmentMand { commitment: elem } )))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Wrapper(u64);
