use crate::{
  data_format::{GraphQlResponseError, JsonRpcResponseError},
  Id,
};
use alloc::{
  boxed::Box,
  string::{String, ToString},
  vec::Vec,
};
use core::fmt::{Debug, Display, Formatter};

#[cfg(feature = "rkyv")]
type RkyvSer = rkyv::ser::serializers::CompositeSerializerError<
  core::convert::Infallible,
  rkyv::ser::serializers::AllocScratchError,
  rkyv::ser::serializers::SharedSerializeMapError,
>;

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
  /// See [core::num::ParseIntError].
  ParseIntErr(core::num::ParseIntError),
  /// See [protobuf::Error]
  #[cfg(feature = "protobuf")]
  Protobuf(protobuf::Error),
  /// See [reqwest::Error]
  #[cfg(feature = "reqwest")]
  Reqwest(reqwest::Error),
  /// A given type couldn't be deserialized.
  RkyvDer(&'static str),
  /// A given type couldn't be serialized.
  #[cfg(feature = "rkyv")]
  RkyvSer(Box<RkyvSer>),
  /// See [serde_json::Error]
  #[cfg(feature = "serde_json")]
  SerdeJson(serde_json::Error),
  /// See [serde_json::Error]
  #[cfg(feature = "serde-xml-rs")]
  SerdeXmlRs(Box<serde_xml_rs::Error>),
  /// See [serde_yaml::Error]
  #[cfg(feature = "serde_yaml")]
  SerdeYaml(serde_yaml::Error),
  /// See [simd_json::Error]
  #[cfg(feature = "simd-json")]
  SimdJson(Box<simd_json::Error>),
  /// See [surf::Error]
  #[cfg(feature = "surf")]
  Surf(Box<surf::Error>),
  /// See [core::num::TryFromIntError]
  TryFromIntError(core::num::TryFromIntError),
  /// See [core::str::Utf8Error]
  Utf8Error(core::str::Utf8Error),
  /// See [wtx::Error]
  #[cfg(feature = "wtx")]
  Wtx(wtx::Error),

  // Internal
  /// A slice-like batch of package is not sorted
  BatchPackagesAreNotSorted,
  /// The server closed the WebSocket connection
  ClosedWsConnection,
  /// A server was not able to receive the full request data after several attempts.
  CouldNotSendTheFullRequestData,
  /// GraphQl response error
  GraphQlResponseError(Vec<GraphQlResponseError<String>>),
  /// The hardware returned an incorrect time value
  IncorrectHardwareTime,
  /// `no_std` has no knowledge of time. Try enabling the `std` feature
  ItIsNotPossibleToUseTimeInNoStd,
  /// JSON-RPC response error
  JsonRpcResultErr(Box<JsonRpcResponseError>),
  /// A variant used to transform `Option`s into `Result`s
  NoInnerValue(&'static str),
  /// A given response id is not present in the set of sent packages.
  ResponseIdIsNotPresentInTheOfSentBatchPackages(Id),
  /// No stored test response to return a result from a request
  TestTransportNoResponse,
  /// It is not possible to convert a `u16` into a HTTP status code
  UnknownHttpStatusCode(u16),
  /// `lucia` can not perform this operation due to known limitations.
  UnsupportedOperation,
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

impl From<core::num::ParseIntError> for Error {
  #[inline]
  fn from(from: core::num::ParseIntError) -> Self {
    Self::ParseIntErr(from)
  }
}

#[cfg(feature = "protobuf")]
impl From<protobuf::Error> for Error {
  #[inline]
  fn from(from: protobuf::Error) -> Self {
    Self::Protobuf(from)
  }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
  #[inline]
  fn from(from: reqwest::Error) -> Self {
    Self::Reqwest(from)
  }
}

#[cfg(feature = "rkyv")]
impl From<&'static str> for Error {
  #[inline]
  fn from(from: &'static str) -> Self {
    Self::RkyvDer(from)
  }
}

#[cfg(feature = "rkyv")]
impl From<RkyvSer> for Error {
  #[inline]
  fn from(from: RkyvSer) -> Self {
    Self::RkyvSer(from.into())
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
    Self::SerdeXmlRs(from.into())
  }
}

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for Error {
  #[inline]
  fn from(from: serde_yaml::Error) -> Self {
    Self::SerdeYaml(from)
  }
}

#[cfg(feature = "simd-json")]
impl From<simd_json::Error> for Error {
  #[inline]
  fn from(from: simd_json::Error) -> Self {
    Self::SimdJson(from.into())
  }
}

#[cfg(feature = "surf")]
impl From<surf::Error> for Error {
  #[inline]
  fn from(from: surf::Error) -> Self {
    Self::Surf(from.into())
  }
}

impl From<core::num::TryFromIntError> for Error {
  #[inline]
  fn from(from: core::num::TryFromIntError) -> Self {
    Self::TryFromIntError(from)
  }
}

impl From<core::str::Utf8Error> for Error {
  #[inline]
  fn from(from: core::str::Utf8Error) -> Self {
    Self::Utf8Error(from)
  }
}

#[cfg(feature = "wtx")]
impl From<wtx::Error> for Error {
  #[inline]
  fn from(from: wtx::Error) -> Self {
    Self::Wtx(from)
  }
}

impl<E> From<Vec<GraphQlResponseError<E>>> for Error
where
  E: Display,
{
  #[inline]
  fn from(from: Vec<GraphQlResponseError<E>>) -> Self {
    Self::GraphQlResponseError(
      from
        .into_iter()
        .map(|elem| GraphQlResponseError {
          extensions: elem.extensions.map(|extension| extension.to_string()),
          locations: elem.locations,
          message: elem.message,
          path: elem.path,
        })
        .collect(),
    )
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
