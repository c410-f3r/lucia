#![allow(
  // Used by some APIs
  dead_code
)]

use core::fmt::Display;

pub(crate) struct QueryParamWriter<'str, S> {
  initial_len: usize,
  s: &'str mut S,
}

impl<'str, S> QueryParamWriter<'str, S>
where
  S: cl_aux::String,
{
  #[inline]
  pub(crate) fn new(s: &'str mut S) -> Self {
    Self { initial_len: s.len(), s }
  }

  #[inline]
  pub(crate) fn write<T>(self, param: &str, value: T) -> crate::Result<Self>
  where
    T: Display,
  {
    if self.s.len() == self.initial_len {
      self.s.write_fmt(format_args!("?{param}={value}"))?;
    } else {
      self.s.write_fmt(format_args!("&{param}={value}"))?;
    }
    Ok(self)
  }

  #[inline]
  pub(crate) fn write_opt<T>(self, param: &str, opt: Option<T>) -> crate::Result<Self>
  where
    T: Display,
  {
    if let Some(value) = opt {
      self.write(param, value)
    } else {
      Ok(self)
    }
  }
}
