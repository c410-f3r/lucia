use crate::blockchain::solana::program::spl_token::{MintAccount, TokenAccount};

/// spl-token has two different types of accounts
#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, serde::Deserialize)]
#[serde(content = "info", rename_all = "camelCase", tag = "type")]
pub enum GenericAccount {
  /// Token account
  Account(TokenAccount),
  /// Mint account
  Mint(MintAccount),
}
