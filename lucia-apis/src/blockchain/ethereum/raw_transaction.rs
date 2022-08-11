use crate::blockchain::ethereum::{Bytes, Transaction};

/// Raw bytes of a signed, but not yet sent transaction
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct RawTransaction {
  /// Signed transaction as raw bytes
  pub raw: Bytes,
  /// Transaction details
  pub tx: Transaction,
}
