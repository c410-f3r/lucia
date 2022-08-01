use crate::api::blockchain::solana::program::spl_token::{Account, MintAccount};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
#[cfg_attr(feature = "serde", serde(content = "info", rename_all = "camelCase", tag = "type"))]
pub enum GenericAccount {
  Account(Account),
  Mint(MintAccount),
}
