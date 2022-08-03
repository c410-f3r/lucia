mod transaction_input;
mod transaction_json;
mod transaction_output;

pub use transaction_input::*;
pub use transaction_json::*;
pub use transaction_output::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Copy, Debug)]
pub enum TransactionEncoding {
  Base58,
  Base64,
  Json,
  JsonParsed,
}
