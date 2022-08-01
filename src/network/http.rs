//! Convenient subset of HTTP parameters. Intended to be only used by HTTP endpoints.

mod status_code;

pub use status_code::*;

use crate::{types::MaxUrlParts, utils::RequestThrottling, CommonParams};
use alloc::vec::Vec;
use arrayvec::ArrayString;

/// Contains variants for a number of common HTTP methods such as GET, POST, etc.
#[derive(Clone, Copy, Debug)]
pub enum Method {
  /// DELETE
  Delete,
  /// GET
  Get,
  /// PATCH
  Patch,
  /// POST
  Post,
  /// PUT
  Put,
}

/// All possible HTTP parameters that an API can use.
#[derive(Debug)]
pub struct ReqParams {
  pub(crate) _headers: Vec<Header<16, 64>>,
  pub(crate) _method: Method,
  pub(crate) _mime_type: Option<MimeType>,
  pub(crate) _url_parts: MaxUrlParts,
  pub(crate) _user_agent: Option<UserAgent>,
}

impl ReqParams {
  /// For example, from `http://localhost`.
  #[inline]
  pub fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self {
      _headers: Vec::new(),
      _method: Method::Get,
      _mime_type: None,
      _url_parts: MaxUrlParts::from_origin(origin)?,
      _user_agent: None,
    })
  }
}

/// A HTTP response returned by a submitted HTTP request.
#[derive(Debug)]
pub struct Response {
  /// See [StatusCode].
  pub status_code: StatusCode,
}

impl Response {
  pub(crate) const _DUMMY: Response = Self { status_code: StatusCode::Ok };
}

impl<'res> From<&'res ()> for &'res Response {
  #[inline]
  fn from(_: &'res ()) -> Self {
    &Response::_DUMMY
  }
}

#[cfg(feature = "reqwest")]
impl<'res> TryFrom<&'res reqwest::Response> for Response {
  type Error = crate::Error;

  #[inline]
  fn try_from(res: &'res reqwest::Response) -> Result<Self, Self::Error> {
    let status_code = <_>::try_from(Into::<u16>::into(res.status()))?;
    Ok(Self { status_code })
  }
}

#[cfg(feature = "surf")]
impl<'res> TryFrom<&'res surf::Response> for Response {
  type Error = crate::Error;

  #[inline]
  fn try_from(res: &'res surf::Response) -> Result<Self, Self::Error> {
    let status_code = <_>::try_from(Into::<u16>::into(res.status()))?;
    Ok(Self { status_code })
  }
}

/// Used to specify the data type that is going to be sent to a counterpart.
#[allow(
  // Remove when 1.64 is stable
  clippy::enum_variant_names
)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum MimeType {
  /// Opaque bytes
  _Bytes,
  /// JSON
  _Json,
  /// Plain text
  _Text,
  /// XML
  _Xml,
}

impl MimeType {
  #[inline]
  pub(crate) fn _as_str(self) -> &'static str {
    match self {
      MimeType::_Bytes => "application/octet-stream",
      MimeType::_Json => "application/json",
      MimeType::_Text => "text/plain",
      MimeType::_Xml => "application/xml",
    }
  }
}

#[derive(Debug)]
pub(crate) struct AllReqParamsMut<'generic> {
  pub(crate) _rp: &'generic mut ReqParams,
  pub(crate) _rt: Option<&'generic mut RequestThrottling>,
}

impl AllReqParamsMut<'_> {
  #[inline]
  pub(crate) fn _reset(&mut self) {
    self._rp._headers.clear();
    self._rp._method = Method::Get;
    self._rp._url_parts._retain_origin();
  }
}

impl<'rm> From<&'rm mut CommonParams<ReqParams, ()>> for AllReqParamsMut<'rm> {
  #[inline]
  fn from(from: &'rm mut CommonParams<ReqParams, ()>) -> AllReqParamsMut<'rm> {
    Self { _rp: &mut from.tp, _rt: None }
  }
}

impl<'rm> From<&'rm mut CommonParams<ReqParams, RequestThrottling>> for AllReqParamsMut<'rm> {
  #[inline]
  fn from(from: &'rm mut CommonParams<ReqParams, RequestThrottling>) -> AllReqParamsMut<'rm> {
    Self { _rp: &mut from.tp, _rt: Some(&mut from.up) }
  }
}

#[derive(Debug)]
pub(crate) struct Header<const K: usize, const V: usize> {
  pub(crate) _key: ArrayString<K>,
  pub(crate) _value: ArrayString<V>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum UserAgent {
  _Mozilla,
}

impl UserAgent {
  #[inline]
  pub(crate) fn _as_str(self) -> &'static str {
    match self {
      Self::_Mozilla => "Mozilla",
    }
  }
}
