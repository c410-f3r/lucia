use core::fmt::Display;

pub(crate) struct _QueryParamWriter<'str, S> {
  initial_len: usize,
  s: &'str mut S,
}

impl<'str, S> _QueryParamWriter<'str, S>
where
  S: cl_aux::String,
{
  #[inline]
  pub(crate) fn _new(s: &'str mut S) -> Self {
    Self { initial_len: s.len(), s }
  }

  #[inline]
  pub(crate) fn _write<T>(self, param: &str, value: T) -> crate::Result<Self>
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
  pub(crate) fn _write_opt<T>(self, param: &str, opt: Option<T>) -> crate::Result<Self>
  where
    T: Display,
  {
    if let Some(value) = opt {
      self._write(param, value)
    } else {
      Ok(self)
    }
  }
}
