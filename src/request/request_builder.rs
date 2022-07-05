use crate::{
  network::TransportParams,
  protocol::{JsonRequest, JsonRpcRequest},
  types::Id,
};
use core::fmt::Debug;

/// Responsible for creating request structures.
#[derive(Debug)]
pub struct RequestBuilder<A> {
  pub(crate) _api: A,
  pub(crate) _id: Id,
  pub(crate) _tp: TransportParams,
}

impl<A> RequestBuilder<A> {
  /// Creates an instance with valid initial inner values.
  #[inline]
  pub fn new(_api: A) -> Self {
    Self { _api, _id: 1, _tp: TransportParams::dummy() }
  }

  /// Mutable reference generally used by [crate::network::Transport] calls.
  #[inline]
  pub fn tp_mut(&mut self) -> &mut TransportParams {
    &mut self._tp
  }

  #[inline]
  pub(crate) fn _json_request<D>(&mut self, data: D) -> JsonRequest<D> {
    let rslt = JsonRequest { data, id: self._id };
    self._id = self._id.wrapping_add(1);
    rslt
  }

  #[inline]
  pub(crate) fn _json_rpc_request<P>(
    &mut self,
    method: &'static str,
    params: P,
  ) -> JsonRpcRequest<P> {
    let rslt = JsonRpcRequest { id: self._id, method, params };
    self._id = self._id.wrapping_add(1);
    rslt
  }
}
