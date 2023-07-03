use alloc::vec::Vec;
use ethabi::Address;
use ethereum_types::H256;

/// Access list item
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListItem {
  /// Accessed address
  pub address: Address,
  /// Accessed storage keys
  pub storage_keys: Vec<H256>,
}
