use serde::{de::Error, Deserialize, Deserializer};

#[inline]
pub(crate) fn deserialize_array_from_base58<'de, D, const N: usize>(
  deserializer: D,
) -> Result<[u8; N], D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  let mut array = [0; N];
  bs58::decode(s)
    .into(&mut array)
    .ok()
    .and_then(|len| {
      if len != N {
        return None;
      }
      Some(())
    })
    .ok_or_else(|| D::Error::custom("Could not deserialize base58 hash string"))?;
  Ok(array)
}
