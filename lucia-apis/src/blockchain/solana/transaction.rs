mod transaction_input;
mod transaction_json;
mod transaction_output;

pub use transaction_input::*;
pub use transaction_json::*;
pub use transaction_output::*;

/// Type that serializes to the string "legacy"
#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Legacy {
  /// Legacy
  Legacy,
}

/// Types of data representation of an transaction.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEncoding {
  /// Represents binary data in alphanumeric text.
  Base58,
  /// Represents binary data in sequences of 24 bits.
  Base64,
  /// Json representation.
  Json,
  /// Json representation with additional metadata.
  JsonParsed,
}

/// Level of transaction.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionDetails {
  /// Signatures and metadata
  Full,
  /// Only signatures
  Signatures,
  /// No additional data
  None,
}

/// Transaction version
#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TransactionVersion {
  /// Legacy
  Legacy(Legacy),
  /// Number
  Number(u8),
}

impl TransactionVersion {
  /// Shortcut of `TransactionVersion::Legacy(Legacy::Legacy)`.
  pub const LEGACY: Self = Self::Legacy(Legacy::Legacy);
}
