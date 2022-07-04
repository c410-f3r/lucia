use crate::{
  api::blockchain::solana::{
    endpoint::{Commitment, JsonRpcResponseResultWithContext},
    MessageInput, Solana,
  },
  utils::{into_rslt, OneMandAndOneOpt},
};
use alloc::{string::String, vec::Vec};

_create_json_rpc_endpoint! {
  Solana;

  "getFeeForMessage" => GetFeeForMessageReq<;;>(
    OneMandAndOneOpt<String, Commitment>
  )

  |raw: JsonRpcResponseResultWithContext<Option<u64>>| -> JsonRpcResponseResultWithContext<crate::Result<u64>> {
    JsonRpcResponseResultWithContext {
      context: raw.context,
      value: into_rslt(raw.value)
    }
  }

  get_fee_for_message(
    buffer: &mut Vec<u8>,
    message: &MessageInput,
    commitment: Option<Commitment>
  ) -> crate::Result<:> {
    bincode::serialize_into(&mut *buffer, message)?;
    let string = base64::encode(&buffer);
    buffer.clear();
    GetFeeForMessageReq(OneMandAndOneOpt(string, commitment))
  }

  Ok
}
