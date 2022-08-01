use crate::api::blockchain::ethereum::{BlockNumber, ValueOrArray};
use alloc::vec::Vec;
use ethereum_types::{H160, H256};

/// Filter
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug, Default)]
pub struct Filter {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  /// Address
  pub(crate) address: Option<ValueOrArray<H160>>,
  /// Block Hash
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate) block_hash: Option<H256>,
  /// From Block
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate) from_block: Option<BlockNumber>,
  /// Limit
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate) limit: Option<usize>,
  /// To Block
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate) to_block: Option<BlockNumber>,
  /// Topics
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate) topics: Option<Vec<Option<ValueOrArray<H256>>>>,
}
