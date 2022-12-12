#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("slotSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotSubscribeReq;

  #[pkg::res_data]
  pub type SlotSubscribeRes = u64;
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("slotUnsubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::_serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type SlotUnsubscribeeRes = bool;
}
