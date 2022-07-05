use crate::network::{HttpMethod, HttpParams};
use arrayvec::ArrayString;

/// All possible parameters specified by a [crate::Request] to be used by any
/// [crate::network::Transport] implementation.
#[derive(Debug)]
pub struct TransportParams {
  pub(crate) _http_params: HttpParams,
}

impl TransportParams {
  #[inline]
  pub(crate) const fn dummy() -> Self {
    Self {
      _http_params: HttpParams {
        _method: HttpMethod::Get,
        _rt: None,
        _url: ArrayString::new_const(),
      },
    }
  }

  #[inline]
  pub(crate) fn _clear(&mut self) {
    *self = Self::dummy();
  }
}
