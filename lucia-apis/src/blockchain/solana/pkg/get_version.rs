#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getVersion")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::SolanaHttpPackagesAux;
  use arrayvec::ArrayString;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetVersionReq;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
  #[derive(Debug)]
  #[pkg::res_data]
  pub struct GetVersionRes {
    /// Software version of solana-core.
    pub solana_core: ArrayString<16>,
    /// Unique identifier of the current software's feature set.
    pub feature_set: u64,
  }
}
