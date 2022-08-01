use crate::api::blockchain::solana::{
  program::spl_token::{TransferCheckedInstruction, TransferInstruction},
  SolanaAddressHashStr, SolanaBlockhashStr, SolanaProgramName, SolanaSignatureHashStr,
  MAX_TRANSACTION_ACCOUNTS_NUM,
};
use alloc::{string::String, vec::Vec};
use arrayvec::{ArrayString, ArrayVec};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum GenericInstructionJson {
  Compiled(CompiledInstructionJson),
  Parsed(InstructionJson),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
pub enum InstructionJsonParsedInfo {
  TransferInstruction(TransferInstruction),
  TransferCheckedInstruction(TransferCheckedInstruction),
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::utils::_deserialize_ignore_any"))]
  Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CompiledInstructionJson {
  pub program_id_index: u8,
  pub accounts: ArrayVec<u8, MAX_TRANSACTION_ACCOUNTS_NUM>,
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
