#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("rootSubscribe")),
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
  pub struct RootSubscribeReqData;

  #[pkg::res_data]
  pub type RootSubscribeResData = u64;
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("rootUnsubscribe")),
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
  pub struct RootUnsubscribeReqData(
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::misc::_serde_ser_as_tuple"))]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type RootUnsubscribeResData = bool;
}
