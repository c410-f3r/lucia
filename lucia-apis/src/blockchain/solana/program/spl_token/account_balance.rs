use crate::misc::_MaxNumberStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct AccountBalance {
  pub amount: _MaxNumberStr,
  pub decimals: u8,
  pub ui_amount_string: _MaxNumberStr,
}
