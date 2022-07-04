use crate::types::Id;
use core::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct JsonRpcRequest<P> {
  pub id: Id,
  pub method: &'static str,
  pub params: P,
}

impl<P> Serialize for JsonRpcRequest<P>
where
  P: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("JsonRpcRequest", 4)?;
    state.serialize_field("jsonrpc", "2.0")?;
    state.serialize_field("method", self.method)?;
    state.serialize_field("params", &self.params)?;
    state.serialize_field("id", &self.id)?;
    state.end()
  }
}

impl<P> Borrow<Id> for JsonRpcRequest<P> {
  #[inline]
  fn borrow(&self) -> &Id {
    &self.id
  }
}

impl<P> Eq for JsonRpcRequest<P> {}

impl<P> Hash for JsonRpcRequest<P> {
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.id.hash(state);
  }
}

impl<P> Ord for JsonRpcRequest<P> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(&other.id)
  }
}

impl<P> PartialEq for JsonRpcRequest<P> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<P> PartialOrd for JsonRpcRequest<P> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.id.cmp(&other.id))
  }
}