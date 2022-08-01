use crate::{data_format::JsonRpcResponseError, network::http::StatusCode, types::Id};
#[cfg(feature = "ethereum")]
use alloc::string::String;
use alloc::{borrow::Cow, boxed::Box};
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
  /// See [alloc::string::FromUtf8Error].
  FromUtf8Error(alloc::string::FromUtf8Error),
  /// See [hex::FromHexError]
  #[cfg(feature = "hex")]
  Hex(hex::FromHexError),
  #[cfg(feature = "ku-coin")]
  /// See [crypto_common::InvalidLength].
  InvalidLength(crypto_common::InvalidLength),
  #[cfg(feature = "miniserde")]
  /// See [miniserde::Error].
  Miniserde(miniserde::Error),
  /// See [primitive_types::Error].
  #[cfg(feature = "ethereum")]
  PrimitiveTypes(primitive_types::Error),
  /// See [reqwest::Error]
  #[cfg(feature = "reqwest")]
  Reqwest(reqwest::Error),
  /// See [serde_json::Error]
  #[cfg(feature = "serde_json")]
  SerdeJson(serde_json::Error),
  /// See [serde_json::Error]
  #[cfg(feature = "serde-xml-rs")]
  SerdeXmlRs(serde_xml_rs::Error),
  /// See [surf::Error]
  #[cfg(feature = "surf")]
  Surf(surf::Error),
  /// See [core::num::TryFromIntError]
  TryFromIntError(core::num::TryFromIntError),
  /// See [tungstenite::Error]
  #[cfg(feature = "tokio-tungstenite")]
  Tungstenite(Box<tungstenite::Error>),
  /// See [core::str::Utf8Error]
  Utf8Error(core::str::Utf8Error),
  #[cfg(feature = "std")]
  /// See [std::env::VarError]
  VarError(std::env::VarError),

  // Features
  //
  // Ethereum
  /// Bad data serialization
  #[cfg(feature = "ethereum")]
  TokensInvalidOutputType(String),

  // Solana
  /// Returned data from counterpart is everything but a spl-token account
  #[cfg(feature = "solana")]
  SolanaAccountIsNotSplToken,
  /// Returned data from counterpart is everything but a spl-token account mint
  #[cfg(feature = "solana")]
  SolanaAccountIsNotSplTokenMint,
  /// Usually means that no signing public key is available in the list of all public keys
  #[cfg(feature = "solana")]
  SolanaInexistentOrOutOfBoundsSignatureIndex(usize, Option<usize>),
  /// Hard-coded behavior specified by the Solana blockchain
  #[cfg(feature = "solana")]
  SolanaMessageCanNotHaveMoreThan240Accounts,
  /// The number of signers is not equal the number os signed signatures
  #[cfg(feature = "solana")]
  SolanaSignersShouldHaveSignedAllTransactionSignatures(usize, usize),

  // Internal
  //
  /// Sequence de-serialization must match sizes
  DifferentSequenceDeserialization(usize),
  /// For third-party dependencies that throws strings errors
  Generic(Cow<'static, str>),
  /// Request was expecting a different HTTP status code.
  IncompatibleStatusCode(StatusCode, StatusCode),
  /// The hardware returned an incorrect time value
  IncorrectHardwareTime,
  /// `no_std` has no knowledge of time. Try enabling the `std` feature
  ItIsNotPossibleToUseTimeInNoStd,
  /// A slice-like batch of requests is not sorted
  JsonRpcRequestsAreNotSorted,
  /// Index is greater than the maximum capacity
  JsonRpcResponseIsNotPresentInAnySentRequest(Id),
  /// JSON-RPC response error
  JsonRpcResultErr(Box<JsonRpcResponseError>),
  /// "Different JSON-RPC ids
  JsonRpcSentIdDiffersFromReceivedId(Id, Id),
  /// A variant used to transform `Option`s into `Result`s
  NoInnerValue(&'static str),
  /// No stored test response to return a result from a request
  TestTransportNoResponse,
  /// It is not possible to convert a `u16` into a HTTP status code
  UnknownHttpStatusCode(u16),
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

impl From<alloc::string::FromUtf8Error> for Error {
  #[inline]
  fn from(from: alloc::string::FromUtf8Error) -> Self {
    Self::FromUtf8Error(from)
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

#[cfg(feature = "ku-coin")]
impl From<crypto_common::InvalidLength> for Error {
  #[inline]
  fn from(from: crypto_common::InvalidLength) -> Self {
    Self::InvalidLength(from)
  }
}

#[cfg(feature = "miniserde")]
impl From<miniserde::Error> for Error {
  #[inline]
  fn from(from: miniserde::Error) -> Self {
    Self::Miniserde(from)
  }
}

#[cfg(feature = "ethereum")]
impl From<primitive_types::Error> for Error {
  #[inline]
  fn from(from: primitive_types::Error) -> Self {
    Self::PrimitiveTypes(from)
  }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
  #[inline]
  fn from(from: reqwest::Error) -> Self {
    Self::Reqwest(from)
  }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
  #[inline]
  fn from(from: serde_json::Error) -> Self {
    Self::SerdeJson(from)
  }
}

#[cfg(feature = "serde-xml-rs")]
impl From<serde_xml_rs::Error> for Error {
  #[inline]
  fn from(from: serde_xml_rs::Error) -> Self {
    Self::SerdeXmlRs(from)
  }
}

#[cfg(feature = "surf")]
impl From<surf::Error> for Error {
  #[inline]
  fn from(from: surf::Error) -> Self {
    Self::Surf(from)
  }
}

impl From<core::num::TryFromIntError> for Error {
  #[inline]
  fn from(from: core::num::TryFromIntError) -> Self {
    Self::TryFromIntError(from)
  }
}

#[cfg(feature = "tokio-tungstenite")]
impl From<tungstenite::Error> for Error {
  #[inline]
  fn from(from: tungstenite::Error) -> Self {
    Self::Tungstenite(from.into())
  }
}

impl From<core::str::Utf8Error> for Error {
  #[inline]
  fn from(from: core::str::Utf8Error) -> Self {
    Self::Utf8Error(from)
  }
}

#[cfg(feature = "std")]
impl From<std::env::VarError> for Error {
  #[inline]
  fn from(from: std::env::VarError) -> Self {
    Self::VarError(from)
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
