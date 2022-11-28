use crate::blockchain::solana::{
  program::spl_token::{TransferCheckedInstruction, TransferInstruction},
  SolanaAddressHashStr, SolanaBlockhashStr, SolanaProgramName, SolanaSignatureHashStr,
};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
#[allow(
  // Only used in tx deserialization that is already boxed
  variant_size_differences
)]
pub enum GenericInstructionJson {
  Compiled(CompiledInstructionJson),
  Parsed(InstructionJson),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum InstructionJsonParsedInfo {
  TransferInstruction(TransferInstruction),
  TransferCheckedInstruction(TransferCheckedInstruction),
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::misc::_deserialize_ignore_any"))]
  Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CompiledInstructionJson {
  pub program_id_index: u8,
  pub accounts: Vec<u8>,
  pub data: String,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct InnerInstructionJson {
  pub index: u8,
  pub instructions: Vec<GenericInstructionJson>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct InstructionJson {
  pub program: Option<SolanaProgramName>,
  pub program_id: SolanaAddressHashStr,
  pub parsed: Option<InstructionJsonParsed>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct InstructionJsonParsed {
  pub info: InstructionJsonParsedInfo,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: ArrayString<32>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MessageJson<AK, I> {
  pub account_keys: Vec<AK>,
  pub instructions: Vec<I>,
  pub recent_blockhash: SolanaBlockhashStr,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MessageJsonAccountKey {
  pub pubkey: SolanaAddressHashStr,
  pub signer: bool,
  pub writable: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransactionJson<AK, I> {
  pub signatures: Vec<SolanaSignatureHashStr>,
  pub message: MessageJson<AK, I>,
}
