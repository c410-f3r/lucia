#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getVoteAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaAddressHashStr, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetVoteAccountsReq<S>(
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetVoteAccountsConfig<S>>,
  );

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetVoteAccountsRes {
    /// Current
    pub current: Vec<GetVoteAccounts>,
    /// Delinquent
    pub delinquent: Vec<GetVoteAccounts>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetVoteAccounts {
    /// Vote account address, as base-58 encoded string
    pub vote_pubkey: SolanaAddressHashStr,
    /// Validator identity, as base-58 encoded string
    pub node_pubkey: SolanaAddressHashStr,
    /// The stake, in lamports, delegated to this vote account and active in this epoch
    pub activated_stake: u64,
    /// whether the vote account is staked for this epoch
    pub epoch_vote_account: bool,
    /// percentage (0-100) of rewards payout owed to the vote account
    pub commission: u8,
    /// Most recent slot voted on by this vote account
    pub last_vote: u64,
    /// History of how many credits earned by the end of each epoch
    pub epoch_credits: Vec<[u64; 3]>,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetVoteAccountsConfig<S> {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Only return results for this validator vote address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote_pubkey: Option<S>,
    /// Do not filter out delinquent validators with no stake
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_unstake_delinquents: Option<bool>,
  }
}
