use crate::api::blockchain::ethereum::{AccessList, Bytes};
use ethabi::Address;
use ethereum_types::{U256, U64};

/// Call contract request (eth_call / eth_estimateGas)
///
/// When using this for `eth_estimateGas`, all the fields
/// are optional. However, for usage in `eth_call` the
/// `to` field must be provided.
#[derive(Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallRequest {
  /// Sender address (None for arbitrary address)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Address>,
  /// To address (None allowed for eth_estimateGas)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Address>,
  /// Supplied gas (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas: Option<U256>,
  /// Gas price (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "gasPrice")]
  pub gas_price: Option<U256>,
  /// Transfered value (None for no transfer)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<U256>,
  /// Data (None for empty data)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<Bytes>,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
  pub transaction_type: Option<U64>,
  /// Access list
  #[serde(rename = "accessList", default, skip_serializing_if = "Option::is_none")]
  pub access_list: Option<AccessList>,
  /// Max fee per gas
  #[serde(rename = "maxFeePerGas", skip_serializing_if = "Option::is_none")]
  pub max_fee_per_gas: Option<U256>,
  /// miner bribe
  #[serde(rename = "maxPriorityFeePerGas", skip_serializing_if = "Option::is_none")]
  pub max_priority_fee_per_gas: Option<U256>,
}

#[cfg(test)]
mod tests {
  use crate::api::blockchain::ethereum::CallRequest;
  use ethereum_types::Address;

  #[test]
  fn should_serialize_call_request() {
    let call_request = CallRequest {
      from: None,
      to: Some(Address::from_low_u64_be(5)),
      gas: Some(21_000.into()),
      gas_price: None,
      value: Some(5_000_000.into()),
      data: Some(hex::decode("010203").unwrap().into()),
      transaction_type: None,
      access_list: None,
      max_fee_per_gas: None,
      max_priority_fee_per_gas: None,
    };
    assert_eq!(
      serde_json::to_string_pretty(&call_request).unwrap(),
      r#"{
  "to": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203"
}"#
    );
  }

  #[test]
  fn should_deserialize_call_request() {
    let serialized = r#"{
  "to": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203"
}"#;
    let deserialized: CallRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.from, None);
    assert_eq!(deserialized.to, Some(Address::from_low_u64_be(5)));
    assert_eq!(deserialized.gas, Some(21_000.into()));
    assert_eq!(deserialized.gas_price, None);
    assert_eq!(deserialized.value, Some(5_000_000.into()));
    assert_eq!(deserialized.data, Some(hex::decode("010203").unwrap().into()));
  }
}
