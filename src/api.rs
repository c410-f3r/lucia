pub mod blockchain;
pub mod exchange;
pub mod game;
pub mod health;
pub mod test_data;

pub trait Api {
  fn from_origin(origin: &str) -> crate::Result<Self>
  where
    Self: Sized;

  fn origin(&self) -> &crate::types::MaxUrl;
}
