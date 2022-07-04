use crate::RequestBuilder;

/// A wrapper around [RequestBuilder] and any given `T: Transport`.
#[derive(Debug)]
pub struct Client<A, T> {
  rb: RequestBuilder<A>,
  trans: T,
}

impl<A, T> Client<A, T> {
  /// Returns an instance creating an internal [RequestBuilder] based on the passed `A`.
  #[inline]
  pub fn new(trans: T, api: A) -> Self {
    Self { rb: RequestBuilder::new(api), trans }
  }

  /// Owned version of [Self::parts_mut].
  #[inline]
  pub fn into_parts(self) -> (RequestBuilder<A>, T) {
    (self.rb, self.trans)
  }

  /// Returns mutable references of the internal [RequestBuilder] and `T: Transport` as a tuple
  /// to avoid borrow check errors.
  #[inline]
  pub fn parts_mut(&mut self) -> (&mut RequestBuilder<A>, &mut T) {
    (&mut self.rb, &mut self.trans)
  }
}
