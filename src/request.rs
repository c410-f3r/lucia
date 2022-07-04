mod request_builder;
mod request_with_aux;
mod requests;

pub use request_builder::*;
pub use request_with_aux::*;
pub use requests::*;

/// All communication between the parties happens through requests or through the structures
/// that implement this trait.
pub trait Request {
  /// Specifies the final data format because sometimes the returned raw data format is not what
  /// is desired for actual usage.
  type ProcessedResponse;
  /// The expected raw data format returned by the counterpart for this request.
  type RawResponse;

  /// Every request has to have an unique identifier
  fn id(&self) -> crate::types::Id;

  /// Fallible custom-logic that will transform [Self::RawResponse] into [Self::ProcessedResponse]
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse>;
}

impl<T> Request for &'_ T
where
  T: Request,
{
  type ProcessedResponse = T::ProcessedResponse;
  type RawResponse = T::RawResponse;

  #[inline]
  fn id(&self) -> crate::types::Id {
    T::id(self)
  }

  #[inline]
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
    T::process_response(raw)
  }
}
