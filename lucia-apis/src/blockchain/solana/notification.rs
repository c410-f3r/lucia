use crate::blockchain::solana::SlotUpdate;

/// Returned data of the `slotSubscribe` notification.
#[derive(Debug, serde::Deserialize)]
pub struct SlotSubscribeNotification {
  /// Parent slot.
  pub parent: u64,
  /// Current slot.
  pub root: u64,
  /// Newly slot value.
  pub slot: u64,
}

/// Returned data of the `slotsUpdatesSubscribe` notification.
#[derive(Debug, serde::Deserialize)]
pub struct SlotsUpdatesNotification {
  /// Parent slot.
  pub parent: u64,
  /// Newly slot value.
  pub slot: u64,
  /// Unix timestamp of the update.
  pub timestamp: i64,
  /// Slot update
  pub r#type: SlotUpdate,
}
