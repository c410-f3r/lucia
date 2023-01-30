#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("sendTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SendTransactionEncoding, SolanaHttpPkgsAux, SolanaTransactionHashStr,
    TransactionInput,
  };
  use alloc::string::String;
  use base64::Engine;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn send_transaction_data(
      &mut self,
      config: Option<SendTransactionConfig>,
      tx: &TransactionInput,
    ) -> crate::Result<SendTransactionReq> {
      self.byte_buffer.clear();
      bincode::serialize_into(&mut self.byte_buffer, tx)?;
      let encoded = if let Some(SendTransactionConfig {
        encoding: Some(SendTransactionEncoding::Base64),
        ..
      }) = config
      {
        base64::engine::general_purpose::STANDARD.encode(&self.byte_buffer)
      } else {
        bs58::encode(&self.byte_buffer).into_string()
      };
      self.byte_buffer.clear();
      Ok(SendTransactionReq(encoded, config))
    }
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SendTransactionReq(
    String,
    #[serde(skip_serializing_if = "Option::is_none")] Option<SendTransactionConfig>,
  );

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct SendTransactionConfig {
    /// Send transaction encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<SendTransactionEncoding>,
    /// Maximum number of times for the RPC node to retry sending the transaction to the leader. If
    /// this parameter not provided, the RPC node will retry the transaction until it is finalized
    /// or until the blockhash expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<usize>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    /// If true, skip the preflight transaction checks
    pub skip_preflight: bool,
  }

  #[pkg::res_data]
  pub type SendTransactionRes = SolanaTransactionHashStr;
}
