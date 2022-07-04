use crate::{
  api::blockchain::solana::{
    endpoint::{JsonRpcResponseResultWithContext, SlotSubscribeNotification},
    Account,
  },
  utils::_deserialize_ignore_any,
};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Notification {
  AccountSubscribe(JsonRpcResponseResultWithContext<Option<Account>>),
  GetRoot(u64),
  GetSlot(SlotSubscribeNotification),
  #[serde(deserialize_with = "_deserialize_ignore_any")]
  Unknown,
}
