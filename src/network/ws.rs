//! Any possible WebSocket parameter. Intended to be only used by WebSocket endpoints.

use crate::{types::MaxUrlParts, CommonParams};
use core::marker::PhantomData;

/// All possible WebSocket parameters that an API can use.
#[derive(Debug)]
pub struct ReqParams {
  pub(crate) _url_parts: MaxUrlParts,
}

impl ReqParams {
  /// For example, from `ws://localhost`.
  #[inline]
  pub fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self { _url_parts: MaxUrlParts::from_origin(origin)? })
  }
}

#[derive(Debug)]
pub(crate) struct ReqParamsMut<'generic> {
  phantom: PhantomData<&'generic ()>,
}

impl<'rm> From<&'rm mut CommonParams<ReqParams, ()>> for ReqParamsMut<'rm> {
  #[inline]
  fn from(_: &'rm mut CommonParams<ReqParams, ()>) -> ReqParamsMut<'rm> {
    Self { phantom: PhantomData }
  }
}
