use alloc::string::String;

/// When a rpc call encounters an error.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcResponseError {
  /// Indicates the error type that occurred.
  pub code: i32,
  /// Short description of the error.
  pub message: String,
}
