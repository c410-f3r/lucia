use crate::{consts::MAX_JSON_RPC_METHOD_LEN, types::Id};
use arrayvec::ArrayString;
use core::{
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct ProcessedJsonRpcResponse<R> {
  pub id: Id,
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  pub result: R,
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
