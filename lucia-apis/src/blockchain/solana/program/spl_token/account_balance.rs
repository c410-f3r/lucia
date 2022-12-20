use crate::misc::MaxNumberStr;

/// Token balance of an SPL Token account.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
  /// Raw balance without decimals, a string representation of u64.
  pub amount: MaxNumberStr,
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// The balance as a string, using mint-prescribed decimals.
  pub ui_amount_string: MaxNumberStr,
}
