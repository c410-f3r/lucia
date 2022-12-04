#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getFeeForMessage")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, MessageInput, SolanaHttpPackagesAux,
  };
  use alloc::string::String;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {
    #[pkg::aux_data]
    fn get_fee_for_message_data(
      &mut self,
      config: Option<GetFeeForMessageConfig>,
      message: &MessageInput,
    ) -> crate::Result<GetFeeForMessageReq> {
      self.byte_buffer.clear();
      bincode::serialize_into(&mut self.byte_buffer, message)?;
      let string = base64::encode(&self.byte_buffer);
      self.byte_buffer.clear();
      Ok(GetFeeForMessageReq(string, config))
    }
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetFeeForMessageReq(String, Option<GetFeeForMessageConfig>);

  #[pkg::res_data]
  pub type GetFeeForMessageRes = JsonRpcResponseResultWithContext<Option<u64>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetFeeForMessageConfig {
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Minimum slot that the request can be evaluated at.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
  }
}
