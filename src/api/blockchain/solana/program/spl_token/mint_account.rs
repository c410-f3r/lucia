use crate::api::blockchain::solana::SolanaAddressHashStr;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintAccount {
  pub decimals: u8,
  pub freeze_authority: Option<SolanaAddressHashStr>,
  pub is_initialized: bool,
  pub mint_authority: Option<SolanaAddressHashStr>,
}
