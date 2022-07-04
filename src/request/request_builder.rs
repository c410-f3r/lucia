use crate::{
  network::{HttpMethod, HttpParams, TransportParams},
  protocol::{JsonRequest, JsonRpcRequest},
  types::Id,
};
use core::fmt::Debug;

/// Responsible for creating request structures.
///
/// Not meant to be created directly, see [crate::Client::new].
#[derive(Debug)]
pub struct RequestBuilder<A> {
  pub(crate) _api: A,
  pub(crate) _id: Id,
  pub(crate) _tp: TransportParams,
}

impl<A> RequestBuilder<A> {
  #[inline]
  pub(crate) fn new(api: A) -> Self {
    Self {
      _api: api,
      _id: 1,
      _tp: TransportParams {
        http_params: HttpParams { method: HttpMethod::Get, url: <_>::default() },
      },
    }
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
