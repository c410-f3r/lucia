#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("accountSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::{AccountEncoding, Commitment, SolanaWsPkgsAux};
  use lucia::misc::AsyncTrait;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct AccountSubscribeReq<S>(
    #[pkg::field(name = "pk")] S,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "config")]
    Option<AccountSubscribeConfig>,
  )
  where
    S: AsyncTrait;

  #[pkg::res_data]
  pub type AccountSubscribeRes = u64;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct AccountSubscribeConfig {
    /// Account encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
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
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct AccountUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type AccountUnsubscribeRes = bool;
}
