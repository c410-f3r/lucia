use alloc::vec::Vec;
use ethabi::Address;
use ethereum_types::H256;

/// Access list item
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct AccessListItem {
  /// Accessed address
  pub address: Address,
  /// Accessed storage keys
  pub storage_keys: Vec<H256>,
}
