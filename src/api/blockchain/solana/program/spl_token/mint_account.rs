#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintAccount {
  pub decimals: u8,
  pub is_initialized: bool,
  pub supply: u64,
}
