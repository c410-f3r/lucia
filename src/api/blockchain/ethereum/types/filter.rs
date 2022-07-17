use crate::api::blockchain::ethereum::{BlockNumber, ValueOrArray};
use alloc::vec::Vec;
use ethereum_types::{H160, H256};

/// Filter
#[derive(Default, Debug, serde::Serialize)]
pub struct Filter {
  /// Address
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) address: Option<ValueOrArray<H160>>,
  /// Block Hash
  #[serde(rename = "blockHash", skip_serializing_if = "Option::is_none")]
  pub(crate) block_hash: Option<H256>,
  /// From Block
  #[serde(rename = "fromBlock", skip_serializing_if = "Option::is_none")]
  pub(crate) from_block: Option<BlockNumber>,
  /// Limit
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) limit: Option<usize>,
  /// To Block
  #[serde(rename = "toBlock", skip_serializing_if = "Option::is_none")]
  pub(crate) to_block: Option<BlockNumber>,
  /// Topics
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) topics: Option<Vec<Option<ValueOrArray<H256>>>>,
}
