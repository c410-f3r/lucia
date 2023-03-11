use crate::blockchain::solana::{
  GenericTransaction, Reward, SolanaAddressHashStr, SolanaSignatureHashStr, TransactionMeta,
  TransactionVersion,
};
use alloc::vec::Vec;

/// A collection of transactions.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransaction {
  /// Transaction metadata
  pub meta: TransactionMeta,
  /// Generic transaction
  pub transaction: GenericTransaction,
  /// Transaction version
  pub version: Option<TransactionVersion>,
}
