use crate::{Id, MAX_JSON_RPC_METHOD_LEN};
use arrayvec::ArrayString;
use core::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

/// Originated from [crate::data_formats::JsonRpcResponse].
#[derive(Debug)]
pub struct ProcessedJsonRpcResponse<R> {
  /// Same as the original
  pub id: Id,
  /// Same as the original
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  /// Transformed result
  pub result: R,
}

impl<P> Borrow<Id> for ProcessedJsonRpcResponse<P> {
  #[inline]
  fn borrow(&self) -> &Id {
    &self.id
  }
}

impl<R> Eq for ProcessedJsonRpcResponse<R> {}

impl<R> Hash for ProcessedJsonRpcResponse<R> {
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.id.hash(state);
  }
}

impl<R> Ord for ProcessedJsonRpcResponse<R> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(&other.id)
  }
}

impl<R> PartialEq for ProcessedJsonRpcResponse<R> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<R> PartialOrd for ProcessedJsonRpcResponse<R> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.id.cmp(&other.id))
  }
}
