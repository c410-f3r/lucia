#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenAccountsByOwner")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext,
    MintOrProgramId, SolanaAddressHashStr, SolanaHttpPackagesAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct GetTokenAccountsByOwnerReqData<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "criteria")] MintOrProgramId<S>,
    #[pkg::field(name = "config")] Option<GetTokenAccountsByOwnerConfigReqData>,
  )
  where
    S: AsRef<str>;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type GetTokenAccountsByOwnerResData =
    JsonRpcResponseResultWithContext<Vec<GetTokenAccountsByOwnerResElem>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetTokenAccountsByOwnerConfigReqData {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetTokenAccountsByOwnerResElem {
    pub account: Account,
    pub pubkey: SolanaAddressHashStr,
  }
}
