use crate::{
  api::blockchain::solana::{
    program::spl_token::GenericAccount, Epoch, SolanaAddressHashStr, SolanaProgramName,
  },
  utils::_deserialize_ignore_any,
};
use alloc::string::String;

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountData {
  Binary(String, AccountEncoding),
  Json(AccountDataJson),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub data: AccountData,
  pub executable: bool,
  pub lamports: u64,
  pub owner: SolanaAddressHashStr,
  pub rent_epoch: Epoch,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDataJson {
  pub parsed: AccountDataJsonParsed,
  pub program: SolanaProgramName,
  pub space: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountDataJsonParsed {
  SplTokenAccount(GenericAccount),
  /// Unknown program
  #[serde(deserialize_with = "_deserialize_ignore_any")]
  Unknown,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountEncoding {
  Base58,
  Base64,
  JsonParsed,
  #[serde(rename = "base64+zstd")]
  Base64Zstd,
}
