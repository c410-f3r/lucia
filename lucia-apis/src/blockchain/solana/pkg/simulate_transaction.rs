#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("simulateTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    AccountEncoding, Commitment, SendTransactionEncoding, SolanaHttpPkgsAux, TransactionInput,
  };
  use base64::Engine;
  use lucia::misc::AsyncBounds;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn simulate_transaction_data<A>(
      &mut self,
      config: Option<SimulateTransactionConfig<A>>,
      tx: &TransactionInput,
    ) -> crate::Result<SimulateTransactionReq<A>>
    where
      A: AsyncBounds,
    {
      self.byte_buffer.clear();
      bincode::serialize_into(&mut self.byte_buffer, tx)?;
      let encoded = if let Some(SimulateTransactionConfig {
        encoding: Some(SendTransactionEncoding::Base64),
        ..
      }) = config
      {
        base64::engine::general_purpose::STANDARD.encode(&self.byte_buffer)
      } else {
        bs58::encode(&self.byte_buffer).into_string()
      };
      self.byte_buffer.clear();
      Ok(SimulateTransactionReq(encoded, config))
    }
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SimulateTransactionReq<A>(String, Option<SimulateTransactionConfig<A>>)
  where
    A: AsyncBounds;

  #[pkg::res_data]
  pub type SimulateTransactionRes = ();

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct SimulateTransactionConfig<A> {
    /// If true the transaction signatures will be verified
    pub sig_verify: bool,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<SendTransactionEncoding>,
    /// Accounts configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<SimulateTransactionAccounts<A>>,
    /// If true, the transaction recent blockhash will be replaced with the most recent blockhash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_recent_blockhash: Option<bool>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }

  /// Accounts configuration
  #[derive(Debug, serde::Serialize)]
  pub struct SimulateTransactionAccounts<A> {
    addresses: A,
    encoding: AccountEncoding,
  }
}
