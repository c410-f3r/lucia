use crate::{
  api::blockchain::solana::{Account, JsonRpcResponseResultWithContext},
  utils::_deserialize_ignore_any,
};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Notification {
  AccountSubscribe(JsonRpcResponseResultWithContext<Option<Account>>),
  GetRoot(u64),
  GetSlot(SlotSubscribeNotification),
  #[serde(deserialize_with = "_deserialize_ignore_any")]
  Unknown,
}

#[derive(Debug, serde::Deserialize)]
pub struct SlotSubscribeNotification {
  pub parent: u64,
  pub root: u64,
  pub slot: u64,
}
