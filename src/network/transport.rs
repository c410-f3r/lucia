use crate::{
  network::TransportParams,
  utils::{decode_many, decode_one},
  Request, Requests,
};
use alloc::boxed::Box;
use bytes::Bytes;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

/// Any means of transferring data between two parties
#[async_trait::async_trait]
pub trait Transport {
  /// Sends a given request without trying to retried any counterpart data.
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync;

  /// Sends a request and then awaits its counterpart data
  async fn send_and_retrieve<T>(
    &mut self,
    req: T,
    tp: &mut TransportParams,
  ) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync;

  /// Convenient method used for JSON-RPC 2.0 batch requests
  #[inline]
  async fn send_retrieve_and_decode_many<BUFFER, REQS>(
    &mut self,
    buffer: &mut BUFFER,
    requests: &REQS,
    tp: &mut TransportParams,
  ) -> crate::Result<REQS::Output>
  where
    BUFFER: Send,
    REQS: Debug + Requests<BUFFER> + Send + Serialize + Sync,
  {
    let bytes = &if let (0, Some(0)) = requests.size_hint() {
      Bytes::new()
    } else {
      self.send_and_retrieve(requests, tp).await?
    };
    decode_many(buffer, bytes, requests)
  }

  /// Similar to [send_retrieve_and_decode_many] but expects a single data return from the
  /// counterpart.
  #[inline]
  async fn send_retrieve_and_decode_one<REQ>(
    &mut self,
    req: &REQ,
    tp: &mut TransportParams,
  ) -> crate::Result<REQ::ProcessedResponse>
  where
    REQ: Debug + Request + Send + Serialize + Sync,
    REQ::RawResponse: Debug + for<'de> Deserialize<'de>,
  {
    let bytes = self.send_and_retrieve(req, tp).await?;
    decode_one(&bytes, req)
  }
}
