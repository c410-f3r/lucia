/// A convenient wrapper intended for anything that mutable dereferences to [crate::req_res::RequestManager]
/// along side any given `T: Transport`.
///
/// If desired, you can create your own set of APIs or transports by directly calling
/// their constructors.
///
/// # Types
///
/// * `RM`: `R`equest `M`anager
/// * `T`: `T`ransport
#[derive(Debug)]
pub struct Pair<RM, T> {
  /// See [crate::req_res::RequestManager].
  pub rm: RM,
  /// See [crate::network::Transport].
  pub trans: T,
}

impl<RM, T> Pair<RM, T>
where
  RM: Send,
{
  /// Constructor shortcut.
  #[inline]
  pub const fn new(rm: RM, trans: T) -> Self {
    Self { rm, trans }
  }

  /// Owned version of [Self::parts_mut].
  #[inline]
  pub fn into_parts(self) -> (RM, T) {
    (self.rm, self.trans)
  }

  /// Another shortcut to easy development.
  #[inline]
  pub fn parts_mut(&mut self) -> (&mut RM, &mut T) {
    (&mut self.rm, &mut self.trans)
  }
}
