use crate::{
  protocol::{JsonRequest, JsonRpcRequest},
  types::Id,
};
use core::{fmt::Debug, marker::PhantomData};

/// Responsible for creating request structures and orchestrating how requests will be issued
/// to a counterpart.
///
/// # Types
///
/// * `A`: **A**PI
/// * `CP`: **C**ommon **P**arameters
#[derive(Debug)]
pub struct RequestManager<A, CP> {
  pub(crate) _cp: CP,
  pub(crate) _id: Id,
  _phantom: PhantomData<A>,
}

impl<A, CP> RequestManager<A, CP> {
  /// Creates an instance with valid initial inner values.
  ///
  /// # Parameters
  ///
  /// * `cp`: Common parameters that can involve network or custom user parameters. Used and
  ///         modified across all related operations.
  #[inline]
  pub const fn new(cp: CP) -> Self {
    Self { _id: 1, _cp: cp, _phantom: PhantomData }
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
