/// Slot updates which can be used for tracking the live progress of a cluster.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum SlotUpdate {
  /// Connected node received the first shred of a block. Indicates that a new block that is being
  /// produced.
  FirstShredReceived,
  /// Connected node has received all shreds of a block. Indicates a block was recently produced.
  Completed,
  /// The connected node has started validating this block.
  CreatedBank,
  /// The connected node has validated this block.
  Frozen,
  /// The connected node failed to validate this block.
  Dead,
  /// Block was optimistically confirmed by the cluster. It is not guaranteed that an optimistic
  /// confirmation notification will be sent for every finalized blocks.
  OptimisticConfirmation,
  /// The connected node rooted this block.
  Root,
}
