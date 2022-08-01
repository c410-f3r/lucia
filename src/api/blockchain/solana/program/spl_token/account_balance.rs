use crate::types::MaxNumberStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct AccountBalance {
  pub amount: MaxNumberStr,
  pub decimals: u8,
  pub ui_amount_string: MaxNumberStr,
}
