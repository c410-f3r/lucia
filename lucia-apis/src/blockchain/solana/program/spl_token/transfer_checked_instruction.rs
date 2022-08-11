use crate::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransferCheckedInstruction {
  pub authority: SolanaAddressHashStr,
  pub destination: SolanaAddressHashStr,
  pub mint: SolanaAddressHashStr,
  pub source: SolanaAddressHashStr,
  pub token_amount: AccountBalance,
}
