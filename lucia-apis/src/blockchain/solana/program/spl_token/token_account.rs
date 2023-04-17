use crate::blockchain::solana::{
  program::spl_token::{AccountBalance, AccountState},
  SolanaAddressHashStr,
};

/// Holds a certain amount of tokens issued by a mint.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccount {
  /// Base58 identifier
  pub mint: SolanaAddressHashStr,
  /// State
  pub state: AccountState,
  /// Balance
  #[serde(alias = "uiTokenAmount")]
  pub token_amount: AccountBalance,
}
