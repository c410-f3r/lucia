#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum SlotUpdate {
  FirstShredReceived,
  Completed,
  CreatedBank,
  Frozen,
  Dead,
  OptimisticConfirmation,
  Root,
}
