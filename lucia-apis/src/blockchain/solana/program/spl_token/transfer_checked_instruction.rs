use crate::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

/// Data related to the checked transfer instruction.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferCheckedInstruction {
  /// Signing Base58 identifier.
  pub authority: SolanaAddressHashStr,
  /// Receiving Base58 identifier.
  pub destination: SolanaAddressHashStr,
  /// Base58 identifier.
  pub mint: SolanaAddressHashStr,
  /// Sending Base58 identifier.
  pub source: SolanaAddressHashStr,
  /// Balance information;
  pub token_amount: AccountBalance,
}
