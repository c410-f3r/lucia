#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenSupply")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    blockchain::solana::{Commitment, JsonRpcResponseResultWithContext, SolanaHttpPkgsAux},
    misc::MaxNumberStr,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetTokenSupplyReq<S>(
    #[pkg::field(name = "address")] S,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTokenSupplyConfig>,
  );

  #[pkg::res_data]
  pub type GetTokenSupplyRes = JsonRpcResponseResultWithContext<GetTokenSupply>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenSupply {
    /// The raw total token supply without decimal.
    pub amount: MaxNumberStr,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// The total token supply as a string, using mint-prescribed decimals
    pub ui_amount_string: MaxNumberStr,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenSupplyConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
