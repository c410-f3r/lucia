#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getAccountInfo")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext,
    SolanaHttpPackagesAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetAccountInfoReq<S>(
    #[pkg::field(name = "pk")] S,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "config")]
    Option<GetAccountInfoConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetAccountInfoRes = JsonRpcResponseResultWithContext<Option<Account>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetAccountInfoConfig {
    /// Account encoding.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    /// Commitment.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Data slice.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }
}
