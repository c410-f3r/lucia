use crate::types::Id;
use core::{
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

/// Any opaque and generic JSON request
#[derive(Debug, serde::Serialize)]
#[serde(transparent)]
pub struct JsonRequest<D> {
  /// Data that will be sent or retrieved
  pub data: D,
  /// Id used for internal identification. Not deserialized nor serialized.
  #[serde(skip)]
  pub id: Id,
}

impl<D> Eq for JsonRequest<D> {}

impl<D> Hash for JsonRequest<D> {
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.id.hash(state);
  }
}

impl<D> Ord for JsonRequest<D> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(&other.id)
  }
}

impl<D> PartialEq for JsonRequest<D> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<D> PartialOrd for JsonRequest<D> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
