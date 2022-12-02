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
    ) -> crate::Result<SendTransactionReqData> {
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
      Ok(SendTransactionReqData(encoded, config))
    }
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct SendTransactionReqData(String, Option<SendTransactionConfig>);

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct SendTransactionConfig {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<SendTransactionEncoding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_retries: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub preflight_commitment: Option<Commitment>,
    pub skip_preflight: bool,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub enum SendTransactionEncoding {
    Base58,
    Base64,
  }

  #[pkg::res_data]
  pub type SendTransactionResData = SolanaTransactionHashStr;
}
