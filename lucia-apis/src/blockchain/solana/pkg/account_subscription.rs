#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("accountSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::{AccountEncoding, Commitment, SolanaWsPackagesAux};

  #[pkg::aux]
  impl<DRSR> SolanaWsPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct AccountSubscribeReqData<S>(
    #[pkg::field(name = "pk")] S,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "config")]
    Option<AccountSubscribeConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type AccountSubscribeResData = u64;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct AccountSubscribeConfig {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
  }
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("accountUnsubscribe")),
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
  pub struct AccountUnsubscribeReqData(
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::misc::_serde_ser_as_tuple"))]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type AccountUnsubscribeResData = bool;
}
