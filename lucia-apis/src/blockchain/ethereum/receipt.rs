use crate::blockchain::ethereum::Log;
use alloc::vec::Vec;
use ethabi::Address;
use ethereum_types::{Bloom, H256, U256, U64};

/// "Receipt" of an executed transaction: details of its execution.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Receipt {
  /// Transaction hash.
  pub transaction_hash: H256,
  /// Index within the block.
  pub transaction_index: U64,
  /// Hash of the block this transaction was included within.
  pub block_hash: Option<H256>,
  /// Number of the block this transaction was included within.
  pub block_number: Option<U64>,
  /// Sender
  /// Note: default address if the client did not return this value
  /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
  #[cfg_attr(feature = "serde", serde(default))]
  pub from: Address,
  /// Recipient (None when contract creation)
  /// Note: Also `None` if the client did not return this value
  /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
  #[cfg_attr(feature = "serde", serde(default))]
  pub to: Option<Address>,
  /// Cumulative gas used within the block after this was executed.
  pub cumulative_gas_used: U256,
  /// Gas used by this transaction alone.
  ///
  /// Gas used is `None` if the the client is running in light client mode.
  pub gas_used: Option<U256>,
  /// Contract address created, or `None` if not a deployment.
  pub contract_address: Option<Address>,
  /// Logs generated within this transaction.
  pub logs: Vec<Log>,
  /// Status: either 1 (success) or 0 (failure).
  pub status: Option<U64>,
  /// State root.
  pub root: Option<H256>,
  /// Logs bloom
  pub logs_bloom: Bloom,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  #[cfg_attr(
    feature = "serde",
    serde(default, rename = "type", skip_serializing_if = "Option::is_none")
  )]
  pub ty: Option<U64>,
  /// Effective gas price
  pub effective_gas_price: U256,
}
