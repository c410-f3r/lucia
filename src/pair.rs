use crate::RequestBuilder;

/// A convenient wrapper around [RequestBuilder] and any given `T: Transport`.
///
/// If desired, you can create your own set of APIs or transports by directly calling
/// their constructors.
#[derive(Debug)]
pub struct Pair<A, T> {
  /// See [RequestBuilder].
  pub rb: RequestBuilder<A>,
  /// See [crate::network::Transport].
  pub trans: T,
}

impl<A, T> Pair<A, T> {
  /// Shortcut that creates an internal [RequestBuilder] based on the passed `A`
  #[inline]
  pub fn new(trans: T, api: A) -> Self {
    Self { rb: RequestBuilder::new(api), trans }
  }

  /// Owned version of [Self::parts_mut].
  #[inline]
  pub fn into_parts(self) -> (RequestBuilder<A>, T) {
    (self.rb, self.trans)
  }

  /// Another shortcut to easy development.
  #[inline]
  pub fn parts_mut(&mut self) -> (&mut RequestBuilder<A>, &mut T) {
    (&mut self.rb, &mut self.trans)
  }
}
