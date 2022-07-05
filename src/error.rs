use crate::types::Id;
use alloc::{borrow::Cow, string::String};
use core::fmt::{Debug, Display, Formatter};

/// All possible errors are grouped here
#[derive(Debug)]
pub enum Error {
  // External
  //
  /// See [base64::DecodeError]
  #[cfg(feature = "solana")]
  Base64(base64::DecodeError),
  /// See [bincode::Error]
  #[cfg(feature = "solana")]
  Bincode(bincode::Error),
  /// See [arrayvec::CapacityError]
  CapacityError(arrayvec::CapacityError<()>),
  /// See [cl_aux::Error]
  ClAux(cl_aux::Error),
  /// See [ed25519_dalek::SignatureError]
  #[cfg(feature = "solana")]
  Ed25519Dalek(ed25519_dalek::SignatureError),
  /// See [ethabi::Error]
  #[cfg(feature = "ethereum")]
  EthAbi(ethabi::Error),
  /// See [core::fmt::Error]
  Fmt(core::fmt::Error),
  /// See [hex::FromHexError]
  #[cfg(feature = "hex")]
  Hex(hex::FromHexError),
  /// See [reqwest::Error]
  #[cfg(feature = "reqwest")]
  Reqwest(reqwest::Error),
  /// See [serde_json::Error]
  SerdeJson(serde_json::Error),
  /// See [surf::Error]
  #[cfg(feature = "surf")]
  Surf(surf::Error),
  /// See [tungstenite::Error]
  #[cfg(feature = "tokio-tungstenite")]
  Tungstenite(Box<tungstenite::Error>),

  // Features
  //
  // Ethereum
  #[cfg(feature = "ethereum")]
  TokensInvalidOutputType(String),

  // Solana
  #[cfg(feature = "solana")]
  SolanaAccountIsNotSplToken,
  /// Usually means that no signing public key is available in the list of all public keys
  #[cfg(feature = "solana")]
  SolanaInexistentOrOutOfBoundsSignatureIndex(usize, Option<usize>),
  #[cfg(feature = "solana")]
  SolanaMessageCanNotHaveMoreThan240Accounts,
  #[cfg(feature = "solana")]
  SolanaSignersShouldHaveSignedAllTransactionSignatures(usize, usize),

  // Internal
  //
  /// Sequence de-serialization must match sizes
  DifferentSequenceDeserialization(usize),
  /// For third-party dependencies that throws strings errors
  Generic(Cow<'static, str>),
  /// A slice-like batch of requests is not sorted
  JsonRpcRequestsAreNotSorted,
  /// Index is greater than the maximum capacity
  JsonRpcResponseIsNotPresentInAnySentRequest(Id),
  /// JSON-RPC response error
  JsonRpcResultErr(String),
  /// "Different JSON-RPC ids
  JsonRpcSentIdDiffersFromReceviedId(Id, Id),
  /// A variant used to transform `Option`s into `Result`s
  NoInnerValue(&'static str),
  /// No stored test response to return a result from a request
  TestTransportNoResponse,
}

#[cfg(feature = "solana")]
impl From<base64::DecodeError> for Error {
  #[inline]
  fn from(from: base64::DecodeError) -> Self {
    Self::Base64(from)
  }
}

#[cfg(feature = "solana")]
impl From<bincode::Error> for Error {
  #[inline]
  fn from(from: bincode::Error) -> Self {
    Self::Bincode(from)
  }
}

impl<T> From<arrayvec::CapacityError<T>> for Error {
  #[inline]
  fn from(_: arrayvec::CapacityError<T>) -> Self {
    Self::CapacityError(arrayvec::CapacityError::new(()))
  }
}

impl From<cl_aux::Error> for Error {
  #[inline]
  fn from(from: cl_aux::Error) -> Self {
    Self::ClAux(from)
  }
}

#[cfg(feature = "ethereum")]
impl From<ethabi::Error> for Error {
  #[inline]
  fn from(from: ethabi::Error) -> Self {
    Self::EthAbi(from)
  }
}

impl From<core::fmt::Error> for Error {
  #[inline]
  fn from(from: core::fmt::Error) -> Self {
    Self::Fmt(from)
  }
}

#[cfg(feature = "solana")]
impl From<ed25519_dalek::SignatureError> for Error {
  #[inline]
  fn from(from: ed25519_dalek::SignatureError) -> Self {
    Self::Ed25519Dalek(from)
  }
}

#[cfg(feature = "hex")]
impl From<hex::FromHexError> for Error {
  #[inline]
  fn from(from: hex::FromHexError) -> Self {
    Self::Hex(from)
  }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
  #[inline]
  fn from(from: reqwest::Error) -> Self {
    Self::Reqwest(from)
  }
}

impl From<serde_json::Error> for Error {
  #[inline]
  fn from(from: serde_json::Error) -> Self {
    Self::SerdeJson(from)
  }
}

#[cfg(feature = "surf")]
impl From<surf::Error> for Error {
  #[inline]
  fn from(from: surf::Error) -> Self {
    Self::Surf(from)
  }
}

#[cfg(feature = "tokio-tungstenite")]
impl From<tungstenite::Error> for Error {
  #[inline]
  fn from(from: tungstenite::Error) -> Self {
    Self::Tungstenite(Box::new(from))
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    <Error as Debug>::fmt(self, f)
  }
}
