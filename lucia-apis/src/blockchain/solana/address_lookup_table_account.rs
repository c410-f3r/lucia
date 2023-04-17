use crate::blockchain::solana::SolanaAddressHash;

/// Address lookup table accounts used in v0 messages
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddressLookupTableAccount {
  /// Identifier
  pub key: SolanaAddressHash,
  /// Accounts
  pub addresses: Vec<SolanaAddressHash>,
}
