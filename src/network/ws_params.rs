use crate::types::MaxUrlParts;

/// All possible WebSocket parameters that an API can use.
#[derive(Debug)]
pub struct WsParams {
  pub(crate) _url_parts: MaxUrlParts,
}

impl WsParams {
  /// For example, from `ws://localhost`.
  #[inline]
  pub fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self { _url_parts: MaxUrlParts::from_origin(origin)? })
  }
}
