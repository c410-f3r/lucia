mod transaction_input;
mod transaction_json;
mod transaction_output;

pub use transaction_input::*;
pub use transaction_json::*;
pub use transaction_output::*;

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
