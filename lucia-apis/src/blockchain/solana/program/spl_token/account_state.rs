/// Account state
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountState {
  /// Frozen
  Frozen,
  /// Initialized
  Initialized,
  /// Uninitialized
  Uninitialized,
}
