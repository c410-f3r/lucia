use crate::network::{HttpMethod, HttpParams};

#[derive(Debug)]
pub struct TransportParams {
  pub(crate) http_params: HttpParams,
}

impl TransportParams {
  #[inline]
  pub(crate) fn _clear(&mut self) {
    self.http_params.method = HttpMethod::Get;
    self.http_params.url.clear();
  }
}
