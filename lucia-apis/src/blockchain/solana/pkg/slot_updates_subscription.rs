#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("slotsUpdatesSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::SolanaWsPackagesAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct SlotsUpdatesSubscribeReqData;

  #[pkg::res_data]
  pub type SlotsUpdatesSubscribeResData = u64;
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("slotsUpdatesUnsubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::SolanaWsPackagesAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct SlotsUpdatesUnsubscribeReqData(
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::misc::_serde_ser_as_tuple"))]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type SlotsUpdatesUnsubscribeResData = bool;
}
