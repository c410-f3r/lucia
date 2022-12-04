/// Block commitment
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum Commitment {
  /// Middle ground between `Processed` and `Finalized`
  Confirmed,
  /// Most reliable
  Finalized,
  /// Lesser reliable
  Processed,
}

/// Used to filter a sequence of bytes
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct DataSlice {
  /// Bytes length
  pub length: usize,
  /// Bytes offset from `length`
  pub offset: usize,
}

/// Used by the `getTokenAccountsByDelegate` and `getTokenAccountsByOwner` endpoints.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcResponseResultContext {
  /// Related response slot
  pub slot: u64,
}

/// Many responses are returned as a grouping of the actual response and the related slot.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcResponseResultWithContext<V> {
  /// Metadata
  pub context: JsonRpcResponseResultContext,
  /// Actual response value
  pub value: V,
}
