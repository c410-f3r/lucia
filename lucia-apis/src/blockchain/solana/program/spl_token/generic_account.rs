use crate::blockchain::solana::program::spl_token::{Account, MintAccount};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(content = "info", rename_all = "camelCase", tag = "type"))]
#[derive(Debug)]
pub enum GenericAccount {
  Account(Account),
  Mint(MintAccount),
}
