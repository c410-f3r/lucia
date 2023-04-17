//! Native instructions

use crate::blockchain::solana::SolanaAddressHashStr;

/// Data related to the transfer instruction.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstruction {
  /// Receiving Base58 identifier
  pub destination: SolanaAddressHashStr,
  /// Transferred lamports
  pub lamports: u64,
  /// Sending Base58 identifier
  pub source: SolanaAddressHashStr,
}
