#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("blockSubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::{
    AccountEncoding, Commitment, SolanaWsPkgsAux, TransactionDetails,
  };
  use lucia::misc::AsyncTrait;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct BlockSubscribeReq<S>(
    #[pkg::field(name = "pk")] BlockSubscribeFilter<S>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "config")]
    Option<BlockSubscribeConfig>,
  )
  where
    S: AsyncTrait + AsRef<str>;

  #[pkg::res_data]
  pub type BlockSubscribeRes = u64;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct BlockSubscribeConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Account encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    /// Transaction details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetails>,
    /// Whether to populate the `rewards` response array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_rewards: Option<bool>,
  }

  /// Criteria for the logs to receive results by account type.
  #[derive(Debug, serde::Serialize)]
  pub enum BlockSubscribeFilter<S> {
    /// Includes all transactions in block
    #[serde(rename = "all")]
    All,
    /// Returns only transactions that mention the provided public key.
    #[serde(rename = "mentionsAccountOrProgram")]
    MentionsAccountOrProgram(S),
  }
}

#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("blockUnsubscribe")),
  error(crate::Error),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::SolanaWsPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaWsPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct BlockUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type BlockUnsubscribeRes = bool;
}
