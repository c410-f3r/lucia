use crate::{
  api::blockchain::solana::{Commitment, Solana, SolanaTransactionHashStr, TransactionInput},
  utils::OneMandAndOneOpt,
};
use alloc::{string::String, vec::Vec};

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

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum SendTransactionEncoding {
  Base58,
  Base64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct SendTransactionParameters {
  pub skip_preflight: bool,
  pub preflight_commitment: Option<Commitment>,
  pub encoding: Option<SendTransactionEncoding>,
  pub max_retries: Option<usize>,
}
