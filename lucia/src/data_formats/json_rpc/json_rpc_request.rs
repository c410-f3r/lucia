use crate::{
  dnsn::Serialize,
  misc::{ByteBuffer, FromErrorTy},
  req_res::{Request, RequestParamsModifier},
  Id,
};
use core::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

/// A rpc call is represented by sending a [JsonRpcRequest] object to a counterpart.
///
/// The `jsonrpc` field is not included because it will always be `2.0`.
#[derive(Debug)]
pub struct JsonRpcRequest<P> {
  /// An identifier established by the Client
  pub id: Id,
  /// A String containing the name of the method to be invoked
  pub method: &'static str,
  /// A Structured value that holds the parameter values to be used during the invocation of the method
  pub params: P,
}

impl<P> FromErrorTy for JsonRpcRequest<P>
where
  P: FromErrorTy,
{
  type Error = P::Error;
}

impl<CP, DRSR, P, REQP, RESP> Request<CP, DRSR, REQP, RESP> for JsonRpcRequest<P>
where
  P: Request<CP, DRSR, REQP, RESP>,
{
  type Data = P::Data;
  type ProcessedResponse = P::ProcessedResponse;
  type RawResponse = P::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self.params.data()
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    P::process(raw, resp)
  }
}

impl<CP, REQP, P> RequestParamsModifier<CP, REQP> for JsonRpcRequest<P>
where
  P: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    P::modify_all_params(cp, reqp)
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

impl<P> Serialize<()> for JsonRpcRequest<P> {
  #[inline]
  fn to_bytes<B>(&self, _: &mut B, _: &mut ()) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "serde")]
mod serde {
  use crate::data_formats::JsonRpcRequest;

  impl<P> serde::Serialize for JsonRpcRequest<P>
  where
    P: serde::Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::ser::Serializer,
    {
      use serde::ser::SerializeStruct;
      let mut state = serializer.serialize_struct("JsonRpcRequest", 4)?;
      state.serialize_field("jsonrpc", "2.0")?;
      state.serialize_field("method", self.method)?;
      state.serialize_field("params", &self.params)?;
      state.serialize_field("id", &self.id)?;
      state.end()
    }
  }
}

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{data_formats::JsonRpcRequest, dnsn::SerdeJson, misc::ByteBuffer};

  impl<P> crate::dnsn::Serialize<SerdeJson> for JsonRpcRequest<P>
  where
    P: serde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut SerdeJson) -> crate::Result<()>
    where
      B: ByteBuffer,
    {
      serde_json::to_writer(bytes, self)?;
      Ok(())
    }
  }
}
