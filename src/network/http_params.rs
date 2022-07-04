use crate::types::MaxUrl;

#[derive(Clone, Copy, Debug)]
pub(crate) struct HttpParams {
  pub(crate) method: HttpMethod,
  pub(crate) url: MaxUrl,
}

impl HttpParams {
  #[inline]
  pub(crate) fn _set(&mut self, method: HttpMethod, url: &MaxUrl) {
    self.method = method;
    if &self.url != url {
      self.url = *url;
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum HttpMethod {
  Get,
  Post,
}
