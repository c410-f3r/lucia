use crate::{
  data_formats::{JsonRequest, JsonRpcRequest, XmlRequest},
  Id,
};

/// Responsible for creating request structures and orchestrating how requests will be issued
/// to a counterpart.
///
/// # Types
///
/// * `A`: `A`PI
/// * `CP`: `C`ommon `P`arameters
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
#[derive(Debug)]
pub struct RequestManager<A, CP, DRSR> {
  /// API instance.
  pub api: A,
  /// Common parameters that can involve network or custom user parameters.
  pub cp: CP,
  /// Deserializer/Serializer instance
  pub drsr: DRSR,
  requests_num: Id,
}

impl<A, CP, DRSR> RequestManager<A, CP, DRSR> {
  /// Creates an instance with valid initial inner values.
  #[inline]
  pub const fn new(api: A, cp: CP, drsr: DRSR) -> Self {
    Self { api, requests_num: 0, cp, drsr }
  }

  /// The current number of issued requests.
  ///
  /// Wraps when a hard-to-happen overflow occurs
  #[inline]
  pub fn requests_num(&self) -> Id {
    self.requests_num
  }

  /// Constructs [JsonRequest] and also increases the number of requests.
  #[inline]
  pub fn json_request<D>(&mut self, data: D) -> JsonRequest<D> {
    self.increase_requests_num();
    JsonRequest { data }
  }

  /// Constructs [JsonRpcRequest] and also increases the number of requests.
  #[inline]
  pub fn json_rpc_request<P>(&mut self, method: &'static str, params: P) -> JsonRpcRequest<P> {
    self.increase_requests_num();
    JsonRpcRequest { id: self.requests_num, method, params }
  }

  /// Constructs [XmlRequest] and also increases the number of requests.
  #[inline]
  pub fn xml_request<D>(&mut self, data: D) -> XmlRequest<D> {
    self.increase_requests_num();
    XmlRequest { data }
  }

  fn increase_requests_num(&mut self) {
    self.requests_num = self.requests_num.wrapping_add(1);
  }
}
