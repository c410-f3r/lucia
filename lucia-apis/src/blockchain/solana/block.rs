use crate::blockchain::solana::{
  GenericTransaction, Reward, SolanaAddressHashStr, SolanaSignatureHashStr, TransactionMeta,
};
use alloc::vec::Vec;

/// A collection of transactions.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Block {
  /// Base58 identifier.
  pub blockhash: SolanaAddressHashStr,
  /// Parent base58 identifier.
  pub previous_blockhash: SolanaAddressHashStr,
  /// The slot index of this block's parent.
  pub parent_slot: u64,
  /// Block transactions.
  pub transactions: Vec<BlockTransaction>,
  /// Signatures corresponding to the transaction order in the block.
  pub signatures: Option<Vec<SolanaSignatureHashStr>>,
  /// Rewards
  pub rewards: Option<Vec<Reward>>,
  /// Estimated production time, as Unix timestamp of when transaction was processed.
  pub block_time: Option<i64>,
  /// The number of blocks beneath this block.
  pub block_height: Option<u64>,
}

/// Groups transaction's data as well as its additional metadata.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct BlockTransaction {
  /// Transaction metadata
  pub meta: TransactionMeta,
  /// Generic transaction
  pub transaction: GenericTransaction,
}
