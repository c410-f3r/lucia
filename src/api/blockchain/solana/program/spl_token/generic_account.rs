use crate::api::blockchain::solana::program::spl_token::{Account, MintAccount};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, serde::Deserialize)]
#[serde(content = "info", rename_all = "camelCase", tag = "type")]
pub enum GenericAccount {
  Account(Account),
  Mint(MintAccount),
}
