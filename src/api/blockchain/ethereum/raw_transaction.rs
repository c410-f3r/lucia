use crate::api::blockchain::ethereum::{Bytes, Transaction};

/// Raw bytes of a signed, but not yet sent transaction
#[derive(Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RawTransaction {
  /// Signed transaction as raw bytes
  pub raw: Bytes,
  /// Transaction details
  pub tx: Transaction,
}
