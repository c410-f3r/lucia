use crate::api::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferCheckedInstruction {
  pub authority: SolanaAddressHashStr,
  pub destination: SolanaAddressHashStr,
  pub mint: SolanaAddressHashStr,
  pub source: SolanaAddressHashStr,
  pub token_amount: AccountBalance,
}
