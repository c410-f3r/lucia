/// Represents condition on minimum block number or block timestamp.
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum TransactionCondition {
  /// Valid at this minimum block number.
  #[serde(rename = "block")]
  Block(u64),
  /// Valid at given unix time.
  #[serde(rename = "time")]
  Timestamp(u64),
}
