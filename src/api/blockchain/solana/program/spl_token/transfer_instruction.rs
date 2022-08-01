use crate::api::blockchain::solana::SolanaAddressHashStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransferInstruction {
  pub destination: SolanaAddressHashStr,
  pub lamports: u64,
  pub source: SolanaAddressHashStr,
}
