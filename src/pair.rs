use crate::RequestManager;

/// A convenient wrapper around [RequestManager] and any given `T: Transport`.
///
/// If desired, you can create your own set of APIs or transports by directly calling
/// their constructors.
///
/// # Types
///
/// * `A`: **A**PI
/// * `CP`: **C**ommon **P**arameters
/// * `T`: **T**ransport
#[derive(Debug)]
pub struct Pair<A, CP, T>
where
  A: Send,
  CP: Send,
{
  /// See [RequestManager].
  pub rm: RequestManager<A, CP>,
  /// See [crate::network::Transport].
  pub trans: T,
}

impl<A, CP, T> Pair<A, CP, T>
where
  A: Send,
  CP: Send,
{
  /// Shortcut that automatically instantiates [RequestManager].
  #[inline]
  pub const fn new(trans: T, cp: CP) -> Self {
    Self { rm: RequestManager::new(cp), trans }
  }

  /// Owned version of [Self::parts_mut].
  #[inline]
  pub fn into_parts(self) -> (RequestManager<A, CP>, T) {
    (self.rm, self.trans)
  }

  /// Another shortcut to easy development.
  #[inline]
  pub fn parts_mut(&mut self) -> (&mut RequestManager<A, CP>, &mut T) {
    (&mut self.rm, &mut self.trans)
  }
}
