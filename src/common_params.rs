/// Common instance parameters used in [crate::RequestManager].
#[derive(Debug)]
pub struct CommonParams<TP, UP> {
  /// Transport parameters (HTTP, WebSocket, ...).
  pub tp: TP,
  /// Custom user parameter
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
