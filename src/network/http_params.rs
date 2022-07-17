use crate::{types::MaxUrlParts, utils::RequestThrottling, CommonParams};
use alloc::vec::Vec;
use arrayvec::ArrayString;

/// All possible HTTP parameters that an API can use.
#[derive(Debug)]
pub struct HttpParams {
  pub(crate) _headers: Vec<HttpHeader<16, 64>>,
  pub(crate) _method: HttpMethod,
  pub(crate) _url_parts: MaxUrlParts,
}

impl HttpParams {
  /// For example, from `http://localhost`.
  #[inline]
  pub fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self {
      _headers: Vec::new(),
      _method: HttpMethod::_Get,
      _url_parts: MaxUrlParts::from_origin(origin)?,
    })
  }
}

#[derive(Debug)]
pub(crate) struct HttpParamsMut<'generic> {
  pub(crate) _headers: &'generic mut Vec<HttpHeader<16, 64>>,
  pub(crate) _method: HttpMethod,
  pub(crate) _rt: Option<&'generic mut RequestThrottling>,
  pub(crate) _url_parts: &'generic mut MaxUrlParts,
}

impl HttpParamsMut<'_> {
  #[inline]
  pub(crate) fn _reset(&mut self) {
    self._headers.clear();
    self._method = HttpMethod::_Get;
    self._url_parts._retain_origin();
  }
}

impl<'rm> From<&'rm mut HttpParams> for HttpParamsMut<'rm> {
  #[inline]
  fn from(from: &'rm mut HttpParams) -> HttpParamsMut<'rm> {
    Self {
      _headers: &mut from._headers,
      _method: from._method,
      _rt: None,
      _url_parts: &mut from._url_parts,
    }
  }
}

impl<'rm> From<&'rm mut CommonParams<HttpParams, RequestThrottling>> for HttpParamsMut<'rm> {
  #[inline]
  fn from(from: &'rm mut CommonParams<HttpParams, RequestThrottling>) -> HttpParamsMut<'rm> {
    Self {
      _headers: &mut from.tp._headers,
      _method: from.tp._method,
      _rt: Some(&mut from.up),
      _url_parts: &mut from.tp._url_parts,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum HttpMethod {
  _Get,
  _Post,
}

impl HttpMethod {
  #[inline]
  pub(crate) fn _as_str(self) -> &'static str {
    match self {
      HttpMethod::_Get => "GET",
      HttpMethod::_Post => "POST",
    }
  }
}

#[derive(Debug)]
pub(crate) struct HttpHeader<const K: usize, const V: usize> {
  pub(crate) _key: ArrayString<K>,
  pub(crate) _value: ArrayString<V>,
}
