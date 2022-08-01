/// Usually defines *how* a request will be sent to a counterpart.
///
/// Response payload is handled by [crate::RequestResponse].
///
/// # Types
///
/// * `CP`: **C**ommon **P**arameters
/// * `REQP`: **REQ**uest **P**arameters
pub trait RequestParamsModifier<CP, REQP> {
  /// The place where all the different parameters are modified for this particular request.
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()>;
}

impl<CP, REQP, T> RequestParamsModifier<CP, REQP> for &'_ T
where
  T: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()> {
    T::modify_all_params(cp, reqp)
  }
}

impl<CP, REQP> RequestParamsModifier<CP, REQP> for () {
  #[inline]
  fn modify_all_params(_: &mut CP, _: REQP) -> crate::Result<()> {
    Ok(())
  }
}
