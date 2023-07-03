#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getLargestAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaAddressHashStr, SolanaHttpPkgsAux,
  };
  use serde::Serialize;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetLargestAccountsReq<S>(
    #[pkg::field(name = "config")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetLargestAccountsConfig<S>>,
  )
  where
    S: Serialize;

  #[pkg::res_data]
  pub type GetLargestAccountsRes = JsonRpcResponseResultWithContext<Vec<GetLargestAccounts>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetLargestAccountsConfig<S> {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Filter results by account type; currently supported: circulating|nonCirculating
    pub filter: Option<S>,
  }

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetLargestAccounts {
    /// Base58 identifier
    pub address: SolanaAddressHashStr,
    /// Lamports
    pub lamports: f64,
  }
}
