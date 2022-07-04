/// A wrapper for network backends.
///
/// Used to create custom constructors or to avoid possible Rust coherence problems.
#[derive(Debug)]
pub struct TransportWrapper<BA> {
  /// Wrapped backend
  pub backend: BA,
}
