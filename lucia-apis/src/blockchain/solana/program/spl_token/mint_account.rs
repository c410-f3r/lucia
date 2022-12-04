use crate::blockchain::solana::SolanaAddressHashStr;

/// Authority that can issue tokens.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MintAccount {
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// Optional authority that freezes token accounts.
  pub freeze_authority: Option<SolanaAddressHashStr>,
  /// If the state has been initialized
  pub is_initialized: bool,
  /// Optional authority used to mint new tokens.
  pub mint_authority: Option<SolanaAddressHashStr>,
}
