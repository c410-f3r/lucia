#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getProgramAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, Filter, SolanaHttpPackagesAux,
  };
  use arrayvec::ArrayString;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetProgramAccountsReqData<'bytes, 'filter, S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")] Option<GetProgramAccountsConfig<'bytes, 'filter>>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetProgramAccountsResData = Vec<GetProgramAccountsResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetProgramAccountsConfig<'bytes, 'filter> {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub filters: Option<&'filter [Filter<'bytes>]>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct GetProgramAccountsResElem {
    pub account: Account,
    pub pubkey: ArrayString<44>,
  }
}
