use crate::{
  consts::MAX_JSON_RPC_METHOD_LEN, protocol::ProcessedJsonRpcResponse, types::Id, Request,
};
use alloc::string::ToString;
use arrayvec::ArrayString;
use core::{fmt, marker::PhantomData};
use serde::{
  de::{self, MapAccess, Visitor},
  ser::SerializeStruct,
  Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug)]
pub struct JsonRpcResponse<R> {
  pub id: Id,
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  pub result: crate::Result<R>,
}

impl<R> JsonRpcResponse<R> {
  #[inline]
  pub(crate) fn _into_processed<REQ, PRR>(
    self,
    cb: impl FnOnce(R) -> PRR,
  ) -> crate::Result<ProcessedJsonRpcResponse<PRR>>
  where
    REQ: Request<RawResponse = Self, ProcessedResponse = ProcessedJsonRpcResponse<PRR>>,
  {
    Ok(ProcessedJsonRpcResponse { id: self.id, method: self.method, result: cb(self.result?) })
  }
}

impl<'de, R> Deserialize<'de> for JsonRpcResponse<R>
where
  R: Deserialize<'de>,
{
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<JsonRpcResponse<R>, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct CustomVisitor<T>(PhantomData<T>);

    impl<'de, R> Visitor<'de> for CustomVisitor<R>
    where
      R: Deserialize<'de>,
    {
      type Value = JsonRpcResponse<R>;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct JsonRpcResponse")
      }

      fn visit_map<V>(self, mut map: V) -> Result<JsonRpcResponse<R>, V::Error>
      where
        V: MapAccess<'de>,
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
                return Err(de::Error::duplicate_field("error"));
              }
              let any: serde_json::Value = map.next_value()?;
              error = Some(any.to_string());
            }
            Field::Id => {
              if id.is_some() {
                return Err(de::Error::duplicate_field("id"));
              }
              id = Some(map.next_value()?);
            }
            Field::JsonRpc => {
              if jsonrpc.is_some() {
                return Err(de::Error::duplicate_field("jsonrpc"));
              }
              jsonrpc = Some(map.next_value::<&str>()?);
            }
            Field::Method => {
              if method.is_some() {
                return Err(de::Error::duplicate_field("method"));
              }
              method = Some(map.next_value()?);
            }
            Field::Result => {
              if result.is_some() {
                return Err(de::Error::duplicate_field("result"));
              }
              result = Some(map.next_value()?);
            }
          }
        }

        if let Some(elem) = jsonrpc {
          if elem != "2.0" {
            return Err(de::Error::custom("JsonRpc version must be 2.0"));
          }
        } else {
          return Err(de::Error::missing_field("jsonrpc"));
        }

        Ok(JsonRpcResponse {
          id: if let Some(elem) = id {
            elem
          } else {
            return Err(de::Error::missing_field("id"));
          },
          method,
          result: if let Some(elem) = error {
            Err(crate::Error::JsonRpcResultErr(elem))
          } else {
            Ok(result.ok_or_else(|| de::Error::missing_field("result"))?)
          },
        })
      }
    }

    const FIELDS: &[&str] = &["error", "result"];
    deserializer.deserialize_struct("JsonRpcResponse", FIELDS, CustomVisitor(PhantomData))
  }
}

// For TestTransport
impl<R> Serialize for JsonRpcResponse<R>
where
  R: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct(" ", 3)?;
    state.serialize_field("jsonrpc", "2.0")?;
    match self.result {
      Err(ref err) => state.serialize_field("error", &err.to_string())?,
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
