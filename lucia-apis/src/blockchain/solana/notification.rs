use crate::blockchain::solana::{Account, JsonRpcResponseResultWithContext, SlotUpdate};

#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum Notification {
  Account(JsonRpcResponseResultWithContext<Option<Account>>),
  SlotsUpdates(SlotsUpdatesNotification),
  GetRoot(u64),
  GetSlot(SlotSubscribeNotification),
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::misc::_deserialize_ignore_any"))]
  Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct SlotSubscribeNotification {
  pub parent: u64,
  pub root: u64,
  pub slot: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct SlotsUpdatesNotification {
  pub parent: u64,
  pub slot: u64,
  pub timestamp: i64,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: SlotUpdate,
}
