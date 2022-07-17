use crate::{
  api::blockchain::solana::{Commitment, Solana, SolanaTransactionHashStr, TransactionInput},
  utils::OneMandAndOneOpt,
};
use alloc::{string::String, vec::Vec};
use core::fmt::Debug;

_create_json_rpc_endpoint! {
  Solana;

  "sendTransaction" => SendTransactionReq<;;>(
    OneMandAndOneOpt<String, SendTransactionParameters>
  )

  |raw: SolanaTransactionHashStr| -> SolanaTransactionHashStr { raw }

  send_transaction(
    buffer: &mut Vec<u8>,
    opt: Option<SendTransactionParameters>,
    transaction: &TransactionInput,
  ) -> crate::Result<:> {
    bincode::serialize_into(&mut *buffer, transaction)?;
    let encoded = if let Some(SendTransactionParameters {
      encoding: Some(SendTransactionEncoding::Base64),
      ..
    }) = opt
    {
      base64::encode(&buffer)
    } else {
      bs58::encode(&buffer).into_string()
    };
    buffer.clear();
    SendTransactionReq(OneMandAndOneOpt(encoded, opt))
  }

  Ok
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SendTransactionEncoding {
  Base58,
  Base64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransactionParameters {
  pub skip_preflight: bool,
  pub preflight_commitment: Option<Commitment>,
  pub encoding: Option<SendTransactionEncoding>,
  pub max_retries: Option<usize>,
}
