use crate::blockchain::ethereum::{BlockNumber, ValueOrArray};
use alloc::vec::Vec;
use ethereum_types::{H160, H256};

/// Filter
#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Address
  pub(crate) address: Option<ValueOrArray<H160>>,
  /// Block Hash
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) block_hash: Option<H256>,
  /// From Block
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) from_block: Option<BlockNumber>,
  /// Limit
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) limit: Option<usize>,
  /// To Block
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) to_block: Option<BlockNumber>,
  /// Topics
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) topics: Option<Vec<Option<ValueOrArray<H256>>>>,
}
