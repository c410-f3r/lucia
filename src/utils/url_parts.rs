use crate::utils::QueryWriter;
use core::{fmt::Arguments, ops::Range};

pub(crate) type UrlPartsArrayString<const N: usize> = UrlParts<arrayvec::ArrayString<N>>;

/// Some APIs must known certain parts of an URL in order to work.
///
/// Constructors do not verify if an URL has valid contents.
#[derive(Debug)]
pub struct UrlParts<S> {
  origin: Range<usize>,
  path: Range<usize>,
  query: Range<usize>,
  url: S,
}

impl<S> UrlParts<S>
where
  S: cl_aux::String,
{
  /// Creates all inner parts from an origin (https://localhost).
  #[inline]
  pub fn from_origin(origin_str: &str) -> crate::Result<Self> {
    Self::from_origin_path_and_query(origin_str, "", "")
  }

  /// Creates all inner parts from an origin (https://localhost) and a path (/api_version/endpoint).
  #[inline]
  pub fn from_origin_and_path(origin_str: &str, path_str: &str) -> crate::Result<Self> {
    Self::from_origin_path_and_query(origin_str, path_str, "")
  }

  /// Creates all inner parts from an origin (https://localhost), a path (/api_version/endpoint)
  /// and a query (?foo=bar)
  #[inline]
  pub fn from_origin_path_and_query(
    origin_str: &str,
    path_str: &str,
    query_str: &str,
  ) -> crate::Result<Self> {
    let until_path = origin_str.len().wrapping_add(path_str.len());
    let until_query = until_path.wrapping_add(query_str.len());

    let origin = 0..origin_str.len();
    let path = origin_str.len()..until_path;
    let query = until_path..until_query;

    Ok(Self {
      origin,
      path,
      query,
      url: {
        let mut s = S::default();
        s.push(origin_str)?;
        s.push(path_str)?;
        s
      },
    })
  }

  /// Creates all inner parts from an URL (https://localhost/api_version/endpoint?foo=bar).
  #[inline]
  pub fn from_url(url: S) -> crate::Result<Self> {
    Ok(Self { origin: <_>::default(), path: <_>::default(), query: <_>::default(), url })
  }

  /// For example, https://localhost
  #[inline]
  pub fn origin(&self) -> &str {
    self.url.as_ref().get(self.origin.clone()).unwrap_or_default()
  }

  /// For example, /api_version/endpoint
  #[inline]
  pub fn path(&self) -> &str {
    self.url.as_ref().get(self.path.clone()).unwrap_or_default()
  }

  /// For example, ?foo=bar
  #[inline]
  pub fn query(&self) -> &str {
    self.url.as_ref().get(self.query.clone()).unwrap_or_default()
  }

  /// See [QueryWriter<'_, S>].
  #[inline]
  pub fn query_writer(&mut self) -> QueryWriter<'_, S> {
    QueryWriter::new(&mut self.url)
  }

  /// Modifies the path of the inner URL erasing any subsequent content.
  #[inline]
  pub fn set_path(&mut self, args: Arguments<'_>) -> crate::Result<()> {
    let start = self.path.start;
    self._retain_origin();
    self.url.write_fmt(args)?;
    self.path = start..self.url.len();
    Ok(())
  }

  /// For example, https://localhost/api_version/endpoint
  #[inline]
  pub fn url(&self) -> &str {
    &self.url
  }

  #[inline]
  pub(crate) fn _retain_origin(&mut self) {
    self._set_ranges(self.origin.clone(), <_>::default(), <_>::default());
    self.url.truncate(self.origin.end);
  }

  #[inline]
  pub(crate) fn _set_ranges(
    &mut self,
    origin: Range<usize>,
    path: Range<usize>,
    query: Range<usize>,
  ) {
    self.origin = origin;
    self.path = path;
    self.query = query;
  }
}
