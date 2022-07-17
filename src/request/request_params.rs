/// Usually defines *how* a request will be sent to a counterpart.
///
/// Data payload is handled by [crate::RequestData].
///
/// # Types
///
/// * `CP`: **C**ommon **P**arameters
/// * `RPD`: **R**equest **P**arameters **D**efinitions
pub trait RequestParams<CP, RPD> {
  /// The place where all the different parameters are modified for this particular request.
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()>;
}

impl<CP, RPD, T> RequestParams<CP, RPD> for &'_ T
where
  T: RequestParams<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    T::modify_all_params(cp, rpd)
  }
}

impl<CP, RPD> RequestParams<CP, RPD> for () {
  #[inline]
  fn modify_all_params(_: &mut CP, _: RPD) -> crate::Result<()> {
    Ok(())
  }
}
