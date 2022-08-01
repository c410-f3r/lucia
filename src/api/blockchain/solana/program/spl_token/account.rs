use crate::api::blockchain::solana::{program::spl_token::AccountBalance, SolanaAddressHashStr};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Account {
  pub mint: SolanaAddressHashStr,
  #[cfg_attr(feature = "serde", serde(alias = "uiTokenAmount"))]
  pub token_amount: AccountBalance,
}
