use crate::api::blockchain::solana::SolanaAddressHashStr;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstruction {
  pub destination: SolanaAddressHashStr,
  pub lamports: u64,
  pub source: SolanaAddressHashStr,
}