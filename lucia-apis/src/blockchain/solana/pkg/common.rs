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

/// Send transaction encoding
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SendTransactionEncoding {
  /// Represents binary data in alphanumeric text.
  Base58,
  /// Represents binary data in sequences of 24 bits.
  Base64,
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
pub enum MintOrProgramId<S> {
  /// Address is the mint of a token
  Mint(S),
  /// Address is a program
  ProgramId(S),
}

/// Skate state related to an account.
#[derive(Debug, serde::Deserialize)]
pub enum StakeActivationState {
  /// Processing activation.
  Activating,
  /// Successful activation.
  Active,
  /// Deactivated.
  Deactivating,
  /// Inactive
  Inactive,
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
