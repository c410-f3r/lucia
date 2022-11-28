use arrayvec::ArrayString;

use crate::blockchain::solana::SolanaAddressHashStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Reward {
  pub pubkey: SolanaAddressHashStr,
  pub lamports: i64,
  pub post_balance: u64,
  pub reward_type: ArrayString<8>,
  pub commission: Option<u8>,
}
