use alloc::borrow::Cow;
use core::fmt::{Debug, Display, Formatter};
use lucia::network::StatusCode;

/// All possible errors are grouped here
#[derive(Debug)]
pub enum Error {
  // External
  /// See [base64::DecodeError].
  #[cfg(feature = "solana")]
  Base64(base64::DecodeError),
  /// See [bincode::Error].
  #[cfg(feature = "solana")]
  Bincode(bincode::Error),
  /// See [arrayvec::CapacityError].
  CapacityError(arrayvec::CapacityError<()>),
  /// See [ed25519_dalek::SignatureError].
  #[cfg(feature = "ed25519-dalek")]
  Ed25519Dalek(ed25519_dalek::SignatureError),
  #[cfg(feature = "ku-coin")]
  /// See [crypto_common::InvalidLength].
  InvalidLength(crypto_common::InvalidLength),
  /// See [lucia::Error].
  Lucia(lucia::Error),
  /// See [core::num::TryFromIntError].
  TryFromIntError(core::num::TryFromIntError),
  #[cfg(feature = "std")]
  /// See [std::env::VarError].
  VarError(std::env::VarError),

  // Features
  //
  // KuCoin
  /// Bullet request returned an empty set of instance servers.
  EmptySetOfInstanceServers,
  /// Unsuccessful request explained in the contained string.
  KuCoinUnsuccessfulRequest(String),

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
  /// For third-party dependencies that throws strings errors
  Generic(Cow<'static, str>),
  /// Header key can not be greater than 65535 bytes
  HeaderKeyIsTooLarge,
  /// Request was expecting a different HTTP status code.
  IncompatibleStatusCode(StatusCode, StatusCode),
  /// A variant used to transform `Option`s into `Result`s
  NoInnerValue(&'static str),
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

#[cfg(feature = "ed25519-dalek")]
impl From<ed25519_dalek::SignatureError> for Error {
  #[inline]
  fn from(from: ed25519_dalek::SignatureError) -> Self {
    Self::Ed25519Dalek(from)
  }
}

#[cfg(feature = "ku-coin")]
impl From<crypto_common::InvalidLength> for Error {
  #[inline]
  fn from(from: crypto_common::InvalidLength) -> Self {
    Self::InvalidLength(from)
  }
}

impl From<lucia::Error> for Error {
  #[inline]
  fn from(from: lucia::Error) -> Self {
    Self::Lucia(from)
  }
}

impl From<core::num::TryFromIntError> for Error {
  #[inline]
  fn from(from: core::num::TryFromIntError) -> Self {
    Self::TryFromIntError(from)
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
