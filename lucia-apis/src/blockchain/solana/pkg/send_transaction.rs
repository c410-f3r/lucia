#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("sendTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SolanaHttpPackagesAux, SolanaTransactionHashStr, TransactionInput,
  };
  use alloc::string::String;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {
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
        base64::encode(&self.byte_buffer)
      } else {
        bs58::encode(&self.byte_buffer).into_string()
      };
      self.byte_buffer.clear();
      Ok(SendTransactionReq(encoded, config))
    }
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct SendTransactionReq(String, Option<SendTransactionConfig>);

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct SendTransactionConfig {
    /// Send transaction encoding
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<SendTransactionEncoding>,
    /// Maximum number of times for the RPC node to retry sending the transaction to the leader. If
    /// this parameter not provided, the RPC node will retry the transaction until it is finalized
    /// or until the blockhash expires.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_retries: Option<usize>,
    /// Minimum slot at which to perform preflight transaction check
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    /// Preflight commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub preflight_commitment: Option<Commitment>,
    /// If true, skip the preflight transaction checks
    pub skip_preflight: bool,
  }

  /// Send transaction encoding
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub enum SendTransactionEncoding {
    /// Represents binary data in alphanumeric text.
    Base58,
    /// Represents binary data in sequences of 24 bits.
    Base64,
  }

  #[pkg::res_data]
  pub type SendTransactionRes = SolanaTransactionHashStr;
}
