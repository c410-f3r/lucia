use crate::api::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub mint: SolanaAddressHashStr,
  #[serde(alias = "uiTokenAmount")]
  pub token_amount: AccountBalance,
}
