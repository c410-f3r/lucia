use crate::blockchain::solana::{
  program::spl_token::GenericAccount, Epoch, SolanaAddressHashStr, SolanaProgramName,
};
use alloc::string::String;

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum AccountData {
  Binary(String, AccountEncoding),
  Json(AccountDataJson),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Account {
  pub data: AccountData,
  pub executable: bool,
  pub lamports: u64,
  pub owner: SolanaAddressHashStr,
  pub rent_epoch: Epoch,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct AccountDataJson {
  pub parsed: AccountDataJsonParsed,
  pub program: SolanaProgramName,
  pub space: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum AccountDataJsonParsed {
  SplTokenAccount(GenericAccount),
  /// Unknown program
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::misc::_deserialize_ignore_any"))]
  Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Copy, Debug)]
pub enum AccountEncoding {
  Base58,
  Base64,
  JsonParsed,
  #[cfg_attr(feature = "serde", serde(rename = "base64+zstd"))]
  Base64Zstd,
}
