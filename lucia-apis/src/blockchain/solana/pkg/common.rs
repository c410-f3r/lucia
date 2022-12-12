/// Block commitment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Commitment {
  /// Middle ground between `Processed` and `Finalized`
  Confirmed,
  /// Most reliable
  Finalized,
  /// Lesser reliable
  Processed,
}

/// Used to filter a sequence of bytes
#[derive(Debug, serde::Serialize)]
pub struct DataSlice {
  /// Bytes length
  pub length: usize,
  /// Bytes offset from `length`
  pub offset: usize,
}

/// Used by the `getTokenAccountsByDelegate` and `getTokenAccountsByOwner` endpoints.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MintOrProgramId<S>
where
  S: AsRef<str>,
{
  /// Address is the mint of a token
  Mint(S),
  /// Address is a program
  ProgramId(S),
}

/// Response metadata
#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcResponseResultContext {
  /// Related response slot
  pub slot: u64,
}

/// Many responses are returned as a grouping of the actual response and the related slot.
#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcResponseResultWithContext<V> {
  /// Metadata
  pub context: JsonRpcResponseResultContext,
  /// Actual response value
  pub value: V,
}
