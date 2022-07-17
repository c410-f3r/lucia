use crate::types::MaxNumberStr;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
  pub amount: MaxNumberStr,
  pub decimals: u8,
  pub ui_amount_string: MaxNumberStr,
}
