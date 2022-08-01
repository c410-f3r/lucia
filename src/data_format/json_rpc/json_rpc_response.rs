use crate::{
  consts::MAX_JSON_RPC_METHOD_LEN,
  data_format::ProcessedJsonRpcResponse,
  dnsn::{Deserialize, Serialize},
  types::Id,
  utils::ByteBuffer,
};
use arrayvec::ArrayString;

/// Replied from a given [crate::data_format::JsonRpcRequest].
///
/// The `jsonrpc` field is not included because `2.0` is always expected.
#[derive(Debug)]
pub struct JsonRpcResponse<R> {
  /// The same value specified in the request.
  pub id: Id,
  /// Optional parameter returns by the counterpart.
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  /// Contains the `result` or the `error` field.
  pub result: crate::Result<R>,
}

impl<R> JsonRpcResponse<R> {
  #[inline]
  pub(crate) fn _into_processed<F, PRR>(self, cb: F) -> crate::Result<ProcessedJsonRpcResponse<PRR>>
  where
    F: FnOnce(R) -> PRR,
  {
    Ok(ProcessedJsonRpcResponse { id: self.id, method: self.method, result: cb(self.result?) })
  }
}

impl<'de, D> Deserialize<'de, ()> for JsonRpcResponse<D>
where
  D: Default,
{
  #[inline]
  fn from_bytes(_: &'de [u8], _: &mut ()) -> crate::Result<Self> {
    Ok(Self { id: 0, method: None, result: Ok(D::default()) })
  }

  #[inline]
  fn seq_from_bytes<F>(_: &'de [u8], _: &mut (), _: F) -> crate::Result<()>
  where
    F: FnMut(Self) -> crate::Result<()>,
  {
    Ok(())
  }
}

impl<D> Serialize<()> for JsonRpcResponse<D> {
  #[inline]
  fn to_bytes<B>(&self, _: &mut B, _: &mut ()) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "serde")]
mod serde {
  use crate::data_format::JsonRpcResponse;
  use alloc::boxed::Box;

  impl<'de, R> serde::Deserialize<'de> for JsonRpcResponse<R>
  where
    R: serde::Deserialize<'de>,
  {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<JsonRpcResponse<R>, D::Error>
    where
      D: serde::de::Deserializer<'de>,
    {
      struct CustomVisitor<T>(core::marker::PhantomData<T>);

      impl<'de, R> serde::de::Visitor<'de> for CustomVisitor<R>
      where
        R: serde::Deserialize<'de>,
      {
        type Value = JsonRpcResponse<R>;

        #[inline]
        fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          formatter.write_str("struct JsonRpcResponse")
        }

        #[inline]
        fn visit_map<V>(self, mut map: V) -> Result<JsonRpcResponse<R>, V::Error>
        where
          V: serde::de::MapAccess<'de>,
        {
          let mut error = None;
          let mut id = None;
          let mut jsonrpc = None;
          let mut method = None;
          let mut result = None;

          while let Some(key) = map.next_key()? {
            match key {
              Field::Error => {
                if error.is_some() {
                  return Err(serde::de::Error::duplicate_field("error"));
                }
                error = Some(map.next_value::<Box<crate::data_format::JsonRpcResponseError>>()?);
              }
              Field::Id => {
                if id.is_some() {
                  return Err(serde::de::Error::duplicate_field("id"));
                }
                id = Some(map.next_value()?);
              }
              Field::JsonRpc => {
                if jsonrpc.is_some() {
                  return Err(serde::de::Error::duplicate_field("jsonrpc"));
                }
                jsonrpc = Some(map.next_value::<&str>()?);
              }
              Field::Method => {
                if method.is_some() {
                  return Err(serde::de::Error::duplicate_field("method"));
                }
                method = Some(map.next_value()?);
              }
              Field::Result => {
                if result.is_some() {
                  return Err(serde::de::Error::duplicate_field("result"));
                }
                result = Some(map.next_value()?);
              }
            }
          }

          if let Some(elem) = jsonrpc {
            if elem != "2.0" {
              return Err(serde::de::Error::custom("JsonRpc version must be 2.0"));
            }
          } else {
            return Err(serde::de::Error::missing_field("jsonrpc"));
          }

          Ok(JsonRpcResponse {
            id: if let Some(elem) = id {
              elem
            } else {
              return Err(serde::de::Error::missing_field("id"));
            },
            method,
            result: if let Some(elem) = error {
              Err(crate::Error::JsonRpcResultErr(elem))
            } else {
              Ok(result.ok_or_else(|| serde::de::Error::missing_field("result"))?)
            },
          })
        }
      }

      const FIELDS: &[&str] = &["error", "result"];
      deserializer.deserialize_struct(
        "JsonRpcResponse",
        FIELDS,
        CustomVisitor(core::marker::PhantomData),
      )
    }
  }

  impl<R> serde::Serialize for JsonRpcResponse<R>
  where
    R: serde::Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::ser::Serializer,
    {
      use serde::ser::SerializeStruct;
      let mut state = serializer.serialize_struct(" ", 3)?;
      state.serialize_field("jsonrpc", "2.0")?;
      match self.result {
        Err(ref err) => {
          state.serialize_field("error", &alloc::string::ToString::to_string(&err))?
        }
        Ok(ref el) => state.serialize_field("result", &el)?,
      }
      state.serialize_field("id", &self.id)?;
      state.end()
    }
  }

  #[derive(serde::Deserialize)]
  #[serde(field_identifier, rename_all = "lowercase")]
  enum Field {
    Error,
    Id,
    JsonRpc,
    Method,
    Result,
  }
}

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{data_format::JsonRpcResponse, dnsn::SerdeJson};

  impl<'de, R> crate::dnsn::Deserialize<'de, SerdeJson> for JsonRpcResponse<R>
  where
    R: for<'serde_de> ::serde::Deserialize<'serde_de>,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut SerdeJson) -> crate::Result<Self> {
      Ok(serde_json::from_slice(bytes)?)
    }

    #[inline]
    fn seq_from_bytes<F>(bytes: &[u8], _: &mut SerdeJson, cb: F) -> crate::Result<()>
    where
      F: FnMut(Self) -> crate::Result<()>,
    {
      use ::serde::de::Deserializer;
      let mut de = serde_json::Deserializer::from_slice(bytes);
      Ok(de.deserialize_seq(crate::utils::seq_visitor::_SeqVisitor::_new(cb))?)
    }
  }

  impl<R> crate::dnsn::Serialize<SerdeJson> for JsonRpcResponse<R>
  where
    R: serde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut SerdeJson) -> crate::Result<()>
    where
      B: crate::utils::ByteBuffer,
    {
      serde_json::to_writer(bytes, self)?;
      Ok(())
    }
  }
}
