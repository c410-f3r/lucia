/// A convenient wrapper intended for anything that mutable dereferences to
/// [crate::package::PackagesAux] along side any given `T: Transport`.
///
/// If desired, you can create your own set of APIs or transports by directly calling
/// their constructors.
///
/// # Types
///
/// * `PA`: PackagesAux
/// * `T`: Transport
#[derive(Debug)]
pub struct Pair<PA, T> {
  /// See [crate::package::PackagesAux].
  pub pkgs_aux: PA,
  /// See [crate::network::transport::Transport].
  pub trans: T,
}

impl<PA, T> Pair<PA, T> {
  /// Constructor shortcut.
  #[inline]
  pub const fn new(pkgs_aux: PA, trans: T) -> Self {
    Self { pkgs_aux, trans }
  }

  /// Owned version of [Self::parts_mut].
  #[inline]
  pub fn into_parts(self) -> (PA, T) {
    (self.pkgs_aux, self.trans)
  }

  /// Another shortcut to easy development.
  #[inline]
  pub fn parts_mut(&mut self) -> (&mut PA, &mut T) {
    (&mut self.pkgs_aux, &mut self.trans)
  }
}
