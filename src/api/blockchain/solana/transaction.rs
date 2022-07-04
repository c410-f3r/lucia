mod transaction_input;
mod transaction_json;
mod transaction_output;

pub use transaction_input::*;
pub use transaction_json::*;
pub use transaction_output::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEncoding {
  Base58,
  Base64,
  Json,
  JsonParsed,
}
