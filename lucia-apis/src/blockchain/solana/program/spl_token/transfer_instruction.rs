use crate::blockchain::solana::SolanaAddressHashStr;

/// Data related to the transfer instruction.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransferInstruction {
  /// Receiving Base58 identifier
  pub destination: SolanaAddressHashStr,
  /// Transferred lamports
  pub lamports: u64,
  /// Sending Base58 identifier
  pub source: SolanaAddressHashStr,
}
