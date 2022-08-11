/// Common instance parameters used in [crate::req_res::RequestManager].
///
/// # Types
///
/// * `TP`: `T`ransport `P`arameters
/// * `UP`: Custom `U`ser `P`arameters
#[derive(Debug)]
pub struct CommonParams<TP, UP> {
  /// Transport parameters like, for example, HTTP headers.
  pub tp: TP,
  /// Custom user parameters like, for example, request limits.
  pub up: UP,
}

impl<TP, UP> CommonParams<TP, UP> {
  /// If desired, feel free to instantiate through fields.
  #[inline]
  pub fn new(tp: TP, up: UP) -> Self {
    Self { tp, up }
  }
}

impl Default for CommonParams<(), ()> {
  #[inline]
  fn default() -> Self {
    Self::new((), ())
  }
}
