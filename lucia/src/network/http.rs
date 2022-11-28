//! Convenient subset of HTTP parameters. Intended to be only used by HTTP endpoints.

mod status_code;

use crate::{misc::UrlString, network::transport::TransportParams};
use alloc::{string::String, vec::Vec};
pub use status_code::*;

#[derive(Debug)]
#[doc = generic_trans_params_doc!()]
pub struct HttpParams(HttpReqParams, HttpResParams);
impl HttpParams {
  /// For example, from `http://localhost`.
  #[inline]
  pub fn from_url(url: &str) -> crate::Result<Self> {
    Ok(Self(
      HttpReqParams {
        headers: HttpHeaders::default(),
        method: HttpMethod::Get,
        mime_type: None,
        url: UrlString::from_url(url.into())?,
        user_agent: None,
      },
      HttpResParams { status_code: StatusCode::Forbidden },
    ))
  }
}
impl TransportParams for HttpParams {
  type ExternalRequestParams = HttpReqParams;
  type ExternalResponseParams = HttpResParams;

  #[inline]
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams) {
    (self.0, self.1)
  }
}

/// Contains variants for a number of common HTTP methods such as GET, POST, etc.
#[derive(Clone, Copy, Debug)]
pub enum HttpMethod {
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

/// Used to specify the data type that is going to be sent to a counterpart.
#[derive(Clone, Copy, Debug)]
pub enum HttpMimeType {
  /// Opaque bytes
  Bytes,
  /// JSON
  Json,
  /// Plain text
  Text,
  /// XML
  Xml,
  /// YAML
  Yaml,
}

impl HttpMimeType {
  #[inline]
  pub(crate) fn _as_str(self) -> &'static str {
    match self {
      HttpMimeType::Bytes => "application/octet-stream",
      HttpMimeType::Json => "application/json",
      HttpMimeType::Text => "text/plain",
      HttpMimeType::Xml => "application/xml",
      HttpMimeType::Yaml => "application/yaml",
    }
  }
}

/// Characteristic string that lets servers and network peers identify a client.
#[derive(Clone, Copy, Debug)]
pub enum HttpUserAgent {
  /// Generic Mozilla
  Mozilla,
}

impl HttpUserAgent {
  #[inline]
  pub(crate) fn _as_str(self) -> &'static str {
    match self {
      Self::Mozilla => "Mozilla",
    }
  }
}

#[derive(Debug)]
#[doc = generic_trans_req_params_doc!("HTTP")]
pub struct HttpReqParams {
  /// Http headers.
  pub headers: HttpHeaders,
  /// Http method.
  pub method: HttpMethod,
  /// MIME type.
  pub mime_type: Option<HttpMimeType>,
  /// URL.
  pub url: UrlString,
  /// User agent.
  pub user_agent: Option<HttpUserAgent>,
}

#[doc = generic_trans_res_params_doc!("HTTP")]
#[derive(Debug)]
pub struct HttpResParams {
  /// Status code.
  pub status_code: StatusCode,
}

impl HttpResParams {
  pub(crate) const _DUMMY: &Self = &Self { status_code: StatusCode::Ok };
}

/// List of pairs sent on every request.
#[derive(Debug, Default)]
pub struct HttpHeaders {
  buffer: String,
  indcs: Vec<(usize, usize)>,
}

impl HttpHeaders {
  /// Clears the internal buffer "erasing" all previously inserted elements.
  #[inline]
  pub fn clear(&mut self) {
    self.buffer.clear();
    self.indcs.clear();
  }

  /// Retrieves all stored elements.
  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
    self.indcs.iter().scan(0, |idx_tracker, &(key_idx, value_idx)| {
      let key_str = self.buffer.get(*idx_tracker..key_idx)?;
      let value_str = self.buffer.get(key_idx..value_idx)?;
      *idx_tracker = value_idx;
      Some((key_str, value_str))
    })
  }

  /// Pushes a new pair of `key` and `value` at the end of the internal buffer.
  #[inline]
  pub fn push(&mut self, key: &str, value: &str) {
    let curr_len = self.buffer.len();
    let key_idx = curr_len.wrapping_add(key.len());
    let value_idx = key_idx.wrapping_add(value.len());
    self.buffer.push_str(key);
    self.buffer.push_str(value);
    self.indcs.push((key_idx, value_idx));
  }
}

#[cfg(test)]
mod tests {
  use crate::network::HttpHeaders;
  use alloc::{vec, vec::Vec};

  #[test]
  fn headers_has_correct_values() {
    let mut headers = HttpHeaders::default();
    headers.push("1", "2");
    assert_eq!(headers.iter().collect::<Vec<_>>(), vec![("1", "2")]);
    headers.push("3", "4");
    assert_eq!(headers.iter().collect::<Vec<_>>(), vec![("1", "2"), ("3", "4")]);
    headers.clear();
    assert_eq!(headers.iter().collect::<Vec<_>>(), vec![]);
  }
}
