use crate::blockchain::solana::SolanaAddressHashStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MintAccount {
  pub decimals: u8,
  pub freeze_authority: Option<SolanaAddressHashStr>,
  pub is_initialized: bool,
  pub mint_authority: Option<SolanaAddressHashStr>,
}
