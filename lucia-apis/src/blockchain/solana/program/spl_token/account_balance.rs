use crate::misc::_MaxNumberStr;

/// Token balance of an SPL Token account.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct AccountBalance {
  /// Raw balance without decimals, a string representation of u64.
  pub amount: _MaxNumberStr,
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// The balance as a string, using mint-prescribed decimals.
  pub ui_amount_string: _MaxNumberStr,
}
