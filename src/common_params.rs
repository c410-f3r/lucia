/// Can be used in [crate::RequestManager] to specify custom common parameters.
#[derive(Debug)]
pub struct CommonParams<TP, UP> {
  /// Transport parameters (HTTP, WebSocket, ...).
  pub tp: TP,
  /// User parameter
  pub up: UP,
}

impl<TP, UP> CommonParams<TP, UP> {
  /// If desired, feel free to instantiate through fields.
  #[inline]
  pub fn new(tp: TP, up: UP) -> Self {
    Self { tp, up }
  }
}
