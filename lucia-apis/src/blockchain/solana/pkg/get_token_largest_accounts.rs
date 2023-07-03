#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenLargestAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    blockchain::solana::{
      Commitment, JsonRpcResponseResultWithContext, SolanaAddressHashStr, SolanaHttpPkgsAux,
    },
    misc::MaxNumberStr,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenLargestAccountsReq<S>(
    #[pkg::field(name = "address")] S,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTokenLargestAccountsConfig>,
  );

  #[pkg::res_data]
  pub type GetTokenLargestAccountsRes =
    JsonRpcResponseResultWithContext<Vec<GetTokenLargestAccounts>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenLargestAccountsConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenLargestAccounts {
    /// Base58 identifier.
    pub address: SolanaAddressHashStr,
    /// The raw token account balance without decimals, a string representation of u64.
    pub amount: MaxNumberStr,
    /// Number of base 10 digits to the right of the decimal place
    pub decimals: u8,
    /// The token account balance as a string, using mint-prescribed decimals.
    pub ui_amount_string: MaxNumberStr,
  }
}
