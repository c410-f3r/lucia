use crate::network::{Transport, TransportParams};
use alloc::{borrow::Cow, boxed::Box, collections::VecDeque, vec::Vec};
use bytes::Bytes;
use core::fmt::Debug;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct Test {
  asserted: usize,
  requests: Vec<Cow<'static, str>>,
  responses: VecDeque<Cow<'static, str>>,
}

impl Test {
  #[inline]
  pub fn assert_does_not_have_non_asserted_requests(&self) {
    assert_eq!(self.asserted, self.requests.len())
  }

  #[cfg(test)]
  #[inline]
  pub fn assert_has_request(&mut self, req: &str) {
    let stored = self.requests.get(self.asserted).unwrap();
    self.asserted = self.asserted.wrapping_add(1);
    assert_eq!(&req, stored);
  }

  #[inline]
  fn pop_response(&mut self) -> crate::Result<Cow<'static, str>> {
    self.responses.pop_front().ok_or(crate::Error::TestTransportNoResponse)
  }

  #[inline]
  pub fn push_response(&mut self, res: Cow<'static, str>) {
    self.responses.push_back(res);
  }

  #[inline]
  pub fn set_responses(&mut self, iter: impl IntoIterator<Item = Cow<'static, str>>) {
    self.responses.clear();
    self.responses.extend(iter);
  }

  #[inline]
  fn store_req<T>(&mut self, req: T) -> crate::Result<()>
  where
    T: Debug + Send + Serialize,
  {
    let serialized = serde_json::to_string(&req)?;
    self.requests.push(serialized.into());
    Ok(())
  }
}

#[async_trait::async_trait]
impl Transport for Test {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <&mut Test>::send(&mut self, req, tp).await
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <&mut Test>::send_and_retrieve(&mut self, req, tp).await
  }
}

#[async_trait::async_trait]
impl Transport for &mut Test {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize,
  {
    tp._clear();
    self.store_req(req)
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize,
  {
    tp._clear();
    self.store_req(req)?;
    let response = self.pop_response()?;
    Ok(response.as_bytes().to_vec().into())
  }
}
