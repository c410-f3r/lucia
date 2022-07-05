//! Collection of built-in APIs

pub mod blockchain;
pub mod exchange;
pub mod game;
pub mod health;
pub mod test_data;

/// Basic structure expected by any API.
pub trait Api {
  /// Custom constructor data.
  type Aux;

  /// Creates a new instance from a server origin and any other auxiliary data.
  fn new(origin: &str, aux: Self::Aux) -> crate::Result<Self>
  where
    Self: Sized;

  /// Useful when dealing with abstract programming since all APIs must return a origin.
  fn origin(&self) -> &crate::types::MaxUrl;
}
