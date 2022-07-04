use crate::api::blockchain::solana::program::spl_token::{Account, MintAccount};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(content = "info", tag = "type", rename_all = "camelCase")]
pub enum GenericAccount {
  Account(Account),
  Mint(MintAccount),
}
