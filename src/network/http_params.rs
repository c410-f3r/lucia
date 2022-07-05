use crate::{types::MaxUrl, utils::RequestThrottling};

#[derive(Clone, Copy, Debug)]
pub(crate) struct HttpParams {
  pub(crate) _method: HttpMethod,
  pub(crate) _rt: Option<RequestThrottling>,
  pub(crate) _url: MaxUrl,
}

impl HttpParams {
  #[inline]
  pub(crate) fn _set(&mut self, method: HttpMethod, rt: Option<RequestThrottling>, url: &MaxUrl) {
    self._method = method;
    self._rt = rt;
    if &self._url != url {
      self._url = *url;
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum HttpMethod {
  Get,
  Post,
}
