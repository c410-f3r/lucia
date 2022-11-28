use crate::{data_format::JsonRpcResponseError, Id};
#[cfg(feature = "tokio-tungstenite")]
use alloc::boxed::Box;
use core::fmt::{Debug, Display, Formatter};

/// All possible errors are grouped here
#[derive(Debug)]
pub enum Error {
  // External
  /// See [cl_aux::Error]
  ClAux(cl_aux::Error),
  /// See [core::fmt::Error]
  Fmt(core::fmt::Error),
  /// See [alloc::string::FromUtf8Error].
  FromUtf8Error(alloc::string::FromUtf8Error),
  /// See [std::io::Error].
  #[cfg(feature = "std")]
  IoError(std::io::Error),
  #[cfg(feature = "miniserde")]
  /// See [miniserde::Error].
  Miniserde(miniserde::Error),
  /// See [reqwest::Error]
  #[cfg(feature = "reqwest")]
  Reqwest(reqwest::Error),
  /// See [serde_json::Error]
  #[cfg(feature = "serde_json")]
  SerdeJson(serde_json::Error),
  /// See [serde_json::Error]
  #[cfg(feature = "serde-xml-rs")]
  SerdeXmlRs(serde_xml_rs::Error),
  /// See [serde_yaml::Error]
  #[cfg(feature = "serde_yaml")]
  SerdeYaml(serde_yaml::Error),
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

  // Internal
  /// Package builder needs request content that wasn't provided.
  AbsentBuilderContent,
  /// Package builder needs request parameters that wasn't provided.
  AbsentBuilderParams,
  /// A slice-like batch of package is not sorted
  BatchPackagesAreNotSorted,
  /// A server was not able to receive the full request data after several attempts.
  CouldNotSendTheFullRequestData,
  /// The hardware returned an incorrect time value
  IncorrectHardwareTime,
  /// `no_std` has no knowledge of time. Try enabling the `std` feature
  ItIsNotPossibleToUseTimeInNoStd,
  /// JSON-RPC response error
  JsonRpcResultErr(JsonRpcResponseError),
  /// A given response id is not present in the set of sent packages.
  ResponseIdIsNotPresentInTheOfSentBatchPackages(Id),
  /// No stored test response to return a result from a request
  TestTransportNoResponse,
  /// It is not possible to convert a `u16` into a HTTP status code
  UnknownHttpStatusCode(u16),
  /// Only append is possible but overwritten is still viable through resetting.
  UrlCanNotOverwriteInitiallySetUrl,
}

impl From<cl_aux::Error> for Error {
  #[inline]
  fn from(from: cl_aux::Error) -> Self {
    Self::ClAux(from)
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

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
  #[inline]
  fn from(from: std::io::Error) -> Self {
    Self::IoError(from)
  }
}

#[cfg(feature = "miniserde")]
impl From<miniserde::Error> for Error {
  #[inline]
  fn from(from: miniserde::Error) -> Self {
    Self::Miniserde(from)
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

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for Error {
  #[inline]
  fn from(from: serde_yaml::Error) -> Self {
    Self::SerdeYaml(from)
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

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
