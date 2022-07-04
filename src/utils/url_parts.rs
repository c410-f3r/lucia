pub type UrlPartsArrayString<const N: usize> = UrlParts<arrayvec::ArrayString<N>>;
pub type UrlPartsRef<'str> = UrlParts<&'str str>;
pub type UrlPartsStatic = UrlParts<&'static str>;
pub type UrlPartsString<'str> = UrlParts<&'str str>;

#[derive(Debug, Default)]
pub struct UrlParts<S> {
  origin: S,
  path: S,
  url: S,
}

impl<S> UrlParts<S>
where
  S: AsRef<str>,
{
  #[inline]
  pub fn from_origin_and_path(origin: &str, path: &str) -> crate::Result<Self>
  where
    S: cl_aux::String,
  {
    Ok(Self {
      origin: {
        let mut s = S::default();
        s.push(origin)?;
        s
      },
      path: {
        let mut s = S::default();
        s.push(path)?;
        s
      },
      url: {
        let mut s = S::default();
        s.push(origin)?;
        s.push(path)?;
        s
      },
    })
  }

  #[inline]
  pub const fn new(origin: S, path: S, url: S) -> Self {
    Self { origin, path, url }
  }

  #[inline]
  pub fn origin(&self) -> &S {
    &self.origin
  }

  #[inline]
  pub fn path(&self) -> &S {
    &self.path
  }

  #[inline]
  pub fn url(&self) -> &S {
    &self.url
  }
}
