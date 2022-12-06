#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("rootSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct RootSubscribeReq;

  #[pkg::res_data]
  pub type RootSubscribeRes = u64;
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("rootUnsubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct RootUnsubscribeReq(
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::misc::_serde_ser_as_tuple"))]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type RootUnsubscribeRes = bool;
}
