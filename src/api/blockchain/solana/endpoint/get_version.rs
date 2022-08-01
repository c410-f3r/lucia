use crate::api::blockchain::solana::Solana;
use arrayvec::ArrayString;

_create_json_rpc_endpoint! {
  Solana;

  "getVersion" => GetVersionReq<;;>

  |raw: GetVersionRes| -> GetVersionRes { raw }

  get_version() { GetVersionReq }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug)]
pub struct GetVersionRes {
  pub solana_core: ArrayString<16>,
  pub feature_set: u64,
}
