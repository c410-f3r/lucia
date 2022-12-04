#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenAccountBalance")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    program::spl_token::AccountBalance, Commitment, JsonRpcResponseResultWithContext,
    SolanaHttpPackagesAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetTokenAccountBalanceReq<S>(
    #[pkg::field(name = "pk")] S,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "config")]
    Option<GetTokenAccountBalanceConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetTokenAccountBalanceRes = JsonRpcResponseResultWithContext<AccountBalance>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetTokenAccountBalanceConfig {
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
  }
}
