#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSupply")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaAddressHashStr, SolanaHttpPkgsAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSupplyReq(
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSupplyConfig>,
  );

  #[pkg::res_data]
  pub type GetSupplyRes = JsonRpcResponseResultWithContext<GetSupply>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSupply {
    /// Total supply in lamports
    pub total: u64,
    /// Circulating supply in lamports
    pub circulating: u64,
    /// Non-circulating supply in lamports
    pub non_circulating: u64,
    /// An array of account addresses of non-circulating accounts, as strings.
    pub non_circulating_accounts: Vec<SolanaAddressHashStr>,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSupplyConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Exclude non circulating accounts list from response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_non_circulating_accounts_list: Option<bool>,
  }
}
