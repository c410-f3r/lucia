use crate::{
  api::blockchain::solana::{
    program::spl_token::{TransferCheckedInstruction, TransferInstruction},
    SolanaAddressHashStr, SolanaBlockhashStr, SolanaProgramName, SolanaSignatureHashStr,
    MAX_TRANSACTION_ACCOUNTS_NUM,
  },
  utils::_deserialize_ignore_any,
};
use alloc::{string::String, vec::Vec};
use arrayvec::{ArrayString, ArrayVec};

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum GenericInstructionJson {
  Compiled(CompiledInstructionJson),
  Parsed(InstructionJson),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum InstructionJsonParsedInfo {
  TransferInstruction(TransferInstruction),
  TransferCheckedInstruction(TransferCheckedInstruction),
  #[serde(deserialize_with = "_deserialize_ignore_any")]
  Unknown,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionJson {
  pub program_id_index: u8,
  pub accounts: ArrayVec<u8, MAX_TRANSACTION_ACCOUNTS_NUM>,
  pub data: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct InnerInstructionJson {
  pub index: u8,
  pub instructions: Vec<GenericInstructionJson>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionJson {
  pub program: Option<SolanaProgramName>,
  pub program_id: SolanaAddressHashStr,
  pub parsed: Option<InstructionJsonParsed>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionJsonParsed {
  pub info: InstructionJsonParsedInfo,
  #[serde(rename = "type")]
  pub ty: ArrayString<32>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageJson<AK, I> {
  pub account_keys: Vec<AK>,
  pub instructions: Vec<I>,
  pub recent_blockhash: SolanaBlockhashStr,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageJsonAccountKey {
  pub pubkey: SolanaAddressHashStr,
  pub signer: bool,
  pub writable: bool,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionJson<AK, I> {
  pub signatures: Vec<SolanaSignatureHashStr>,
  pub message: MessageJson<AK, I>,
}
