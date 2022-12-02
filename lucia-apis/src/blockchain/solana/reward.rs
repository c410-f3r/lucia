use crate::blockchain::solana::SolanaAddressHashStr;
use arrayvec::ArrayString;

/// Transaction-level reward that is populated if requested.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Reward {
  /// Base58 account that received the reward.
  pub pubkey: SolanaAddressHashStr,
  /// Number of reward lamports credited or debited by the account.
  pub lamports: i64,
  /// Account balance in lamports after the reward was applied.
  pub post_balance: u64,
  /// Type of reward that currently is only "rent".
  pub reward_type: ArrayString<8>,
  /// Vote account commission when the reward was credited, only present for voting and staking
  /// rewards.
  pub commission: Option<u8>,
}
