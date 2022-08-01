use crate::api::blockchain::ethereum::{AccessList, Bytes, TransactionCondition};
use ethabi::Address;
use ethereum_types::{U256, U64};

/// Send Transaction Parameters
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransactionRequest {
  /// Sender address
  pub from: Address,
  /// Recipient address (None for contract creation)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub to: Option<Address>,
  /// Supplied gas (None for sensible default)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub gas: Option<U256>,
  /// Gas price (None for sensible default)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub gas_price: Option<U256>,
  /// Transfered value (None for no transfer)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub value: Option<U256>,
  /// Transaction data (None for empty bytes)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub data: Option<Bytes>,
  /// Transaction nonce (None for next available nonce)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub nonce: Option<U256>,
  /// Min block inclusion (None for include immediately)
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub condition: Option<TransactionCondition>,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  #[cfg_attr(
    feature = "serde",
    serde(default, rename = "type", skip_serializing_if = "Option::is_none")
  )]
  pub ty: Option<U64>,
  /// Access list
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub access_list: Option<AccessList>,
  /// Max fee per gas
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub max_fee_per_gas: Option<U256>,
  /// miner bribe
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub max_priority_fee_per_gas: Option<U256>,
}

#[cfg(all(feature = "serde_json", test))]
mod tests {
  use crate::api::blockchain::ethereum::{TransactionCondition, TransactionRequest};
  use ethereum_types::Address;

  #[test]
  fn should_serialize_transaction_request() {
    let tx_request = TransactionRequest {
      from: Address::from_low_u64_be(5),
      to: None,
      gas: Some(21_000.into()),
      gas_price: None,
      value: Some(5_000_000.into()),
      data: Some(hex::decode("010203").unwrap().into()),
      nonce: None,
      condition: Some(TransactionCondition::Block(5)),
      ty: None,
      access_list: None,
      max_fee_per_gas: None,
      max_priority_fee_per_gas: None,
    };

    assert_eq!(
      serde_json::to_string_pretty(&tx_request).unwrap(),
      r#"{
  "from": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203",
  "condition": {
    "block": 5
  }
}"#
    );
  }

  #[test]
  fn should_deserialize_transaction_request() {
    let serialized = r#"{
  "from": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203",
  "condition": {
    "block": 5
  }
}"#;
    let deserialized: TransactionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.from, Address::from_low_u64_be(5));
    assert_eq!(deserialized.to, None);
    assert_eq!(deserialized.gas, Some(21_000.into()));
    assert_eq!(deserialized.gas_price, None);
    assert_eq!(deserialized.value, Some(5_000_000.into()));
    assert_eq!(deserialized.data, Some(hex::decode("010203").unwrap().into()));
    assert_eq!(deserialized.nonce, None);
    assert_eq!(deserialized.condition, Some(TransactionCondition::Block(5)));
  }
}
