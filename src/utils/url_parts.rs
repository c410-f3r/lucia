use core::ops::Range;

pub(crate) type UrlPartsArrayString<const N: usize> = UrlParts<arrayvec::ArrayString<N>>;

/// Some APIs must known certain parts of an URL in order to work.
#[derive(Debug, Default)]
pub struct UrlParts<S> {
  origin: Range<usize>,
  path: Range<usize>,
  url: S,
}

impl<S> UrlParts<S>
where
  S: AsRef<str>,
{
  /// Creates all inner parts from an origin (https://localhost) and a path (/api_version/endpoint)
  #[inline]
  pub fn from_origin_and_path(origin_str: &str, path_str: &str) -> crate::Result<Self>
  where
    S: cl_aux::String,
  {
    let origin = 0..origin_str.len();
    let path = origin.len()..path_str.len();
    let url = {
      let mut s = S::default();
      s.push(origin_str)?;
      s.push(path_str)?;
      s
    };
    Ok(Self { origin, path, url })
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

  /// For example, https://localhost/api_version/endpoint
  #[inline]
  pub fn url(&self) -> &S {
    &self.url
  }
}
