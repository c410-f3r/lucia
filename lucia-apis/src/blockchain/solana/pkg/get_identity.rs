#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getIdentity")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{blockchain::solana::SolanaHttpPkgsAux, misc::MaxAddressHashStr};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetIdentityReq;

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "lowercase")]
  pub struct GetIdentityRes {
    /// Base58 address
    pub identity: MaxAddressHashStr,
  }
}
