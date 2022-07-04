use arrayvec::ArrayString;

#[derive(Debug, serde::Deserialize)]
pub struct GenericDataResponse<T> {
  pub code: ArrayString<8>,
  pub data: T,
}
