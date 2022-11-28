use crate::blockchain::solana::{
  GenericTransaction, Reward, SolanaAddressHashStr, SolanaSignatureHashStr, TransactionMeta,
};
use alloc::vec::Vec;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Block {
  pub blockhash: SolanaAddressHashStr,
  pub previous_blockhash: SolanaAddressHashStr,
  pub parent_slot: u64,
  pub transactions: Vec<BlockTransaction>,
  pub signatures: Option<Vec<SolanaSignatureHashStr>>,
  pub rewards: Option<Vec<Reward>>,
  pub block_time: Option<i64>,
  pub block_height: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct BlockTransaction {
  pub meta: TransactionMeta,
  pub transaction: GenericTransaction,
}
