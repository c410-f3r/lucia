//! # `D`eserializatio`N`/`S`erializatio`N`
//!
//! `lucia` abstracts different serialization/deserialization frameworks to enhance de-coupling,
//! enable choice and improve experimentation.

mod deserialize;
#[cfg(feature = "miniserde")]
mod miniserde;
#[cfg(feature = "serde_json")]
mod serde_json;
#[cfg(feature = "serde-xml-rs")]
mod serde_xml_rs;
mod serialize;

#[cfg(feature = "miniserde")]
pub use self::miniserde::*;
#[cfg(feature = "serde_json")]
pub use self::serde_json::*;
#[cfg(feature = "serde-xml-rs")]
pub use self::serde_xml_rs::*;
pub use deserialize::*;
pub use serialize::*;
