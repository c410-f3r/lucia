/// Represents condition on minimum block number or block timestamp.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionCondition {
  /// Valid at this minimum block number.
  #[cfg_attr(feature = "serde", serde(rename = "block"))]
  Block(u64),
  /// Valid at given unix time.
  #[cfg_attr(feature = "serde", serde(rename = "time"))]
  Timestamp(u64),
}
