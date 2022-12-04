use crate::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

/// Holds a certain amount of tokens issued by a mint.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TokenAccount {
  /// Base58 identifier
  pub mint: SolanaAddressHashStr,
  /// Balance
  #[cfg_attr(feature = "serde", serde(alias = "uiTokenAmount"))]
  pub token_amount: AccountBalance,
}
