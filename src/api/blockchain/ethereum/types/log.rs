use crate::api::blockchain::ethereum::Bytes;
use alloc::{string::String, vec::Vec};
use ethereum_types::{H160, H256, U256, U64};

/// A log produced by a transaction.
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Log {
  /// H160
  pub address: H160,
  /// Topics
  pub topics: Vec<H256>,
  /// Data
  pub data: Bytes,
  /// Block Hash
  #[serde(rename = "blockHash")]
  pub block_hash: Option<H256>,
  /// Block Number
  #[serde(rename = "blockNumber")]
  pub block_number: Option<U64>,
  /// Transaction Hash
  #[serde(rename = "transactionHash")]
  pub transaction_hash: Option<H256>,
  /// Transaction Index
  #[serde(rename = "transactionIndex")]
  pub transaction_index: Option<U64>,
  /// Log Index in Block
  #[serde(rename = "logIndex")]
  pub log_index: Option<U256>,
  /// Log Index in Transaction
  #[serde(rename = "transactionLogIndex")]
  pub transaction_log_index: Option<U256>,
  /// Log Type
  #[serde(rename = "logType")]
  pub log_type: Option<String>,
  /// Removed
  pub removed: Option<bool>,
}

impl Log {
  /// Returns true if the log has been removed.
  #[inline]
  pub fn is_removed(&self) -> bool {
    if let Some(val_removed) = self.removed {
      return val_removed;
    }
    if let Some(ref val_log_type) = self.log_type {
      if val_log_type == "removed" {
        return true;
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use crate::api::blockchain::ethereum::types::log::Log;
  use ethabi::Address;
  use ethereum_types::{H160, H256};

  #[test]
  fn is_removed_removed_true() {
    let log = Log {
      address: Address::from_low_u64_be(1),
      topics: vec![],
      data: Default::default(),
      block_hash: Some(H256::from_low_u64_be(2)),
      block_number: Some(1.into()),
      transaction_hash: Some(H256::from_low_u64_be(3)),
      transaction_index: Some(0.into()),
      log_index: Some(0.into()),
      transaction_log_index: Some(0.into()),
      log_type: None,
      removed: Some(true),
    };
    assert_eq!(true, log.is_removed());
  }

  #[test]
  fn is_removed_removed_false() {
    let log = Log {
      address: H160::from_low_u64_be(1),
      topics: vec![],
      data: Default::default(),
      block_hash: Some(H256::from_low_u64_be(2)),
      block_number: Some(1.into()),
      transaction_hash: Some(H256::from_low_u64_be(3)),
      transaction_index: Some(0.into()),
      log_index: Some(0.into()),
      transaction_log_index: Some(0.into()),
      log_type: None,
      removed: Some(false),
    };
    assert_eq!(false, log.is_removed());
  }

  #[test]
  fn is_removed_log_type_removed() {
    let log = Log {
      address: Address::from_low_u64_be(1),
      topics: vec![],
      data: Default::default(),
      block_hash: Some(H256::from_low_u64_be(2)),
      block_number: Some(1.into()),
      transaction_hash: Some(H256::from_low_u64_be(3)),
      transaction_index: Some(0.into()),
      log_index: Some(0.into()),
      transaction_log_index: Some(0.into()),
      log_type: Some("removed".into()),
      removed: None,
    };
    assert_eq!(true, log.is_removed());
  }

  #[test]
  fn is_removed_log_type_mined() {
    let log = Log {
      address: Address::from_low_u64_be(1),
      topics: vec![],
      data: Default::default(),
      block_hash: Some(H256::from_low_u64_be(2)),
      block_number: Some(1.into()),
      transaction_hash: Some(H256::from_low_u64_be(3)),
      transaction_index: Some(0.into()),
      log_index: Some(0.into()),
      transaction_log_index: Some(0.into()),
      log_type: Some("mined".into()),
      removed: None,
    };
    assert_eq!(false, log.is_removed());
  }

  #[test]
  fn is_removed_log_type_and_removed_none() {
    let log = Log {
      address: Address::from_low_u64_be(1),
      topics: vec![],
      data: Default::default(),
      block_hash: Some(H256::from_low_u64_be(2)),
      block_number: Some(1.into()),
      transaction_hash: Some(H256::from_low_u64_be(3)),
      transaction_index: Some(0.into()),
      log_index: Some(0.into()),
      transaction_log_index: Some(0.into()),
      log_type: None,
      removed: None,
    };
    assert_eq!(false, log.is_removed());
  }
}