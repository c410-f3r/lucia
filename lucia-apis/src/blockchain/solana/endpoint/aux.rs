use crate::blockchain::solana::AccountEncoding;

/// Block commitment
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum Commitment {
  // Middle ground
  Confirmed,
  // Most reliable
  Finalized,
  // Lesser reliable
  Processed,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CommitmentMand {
  pub commitment: Commitment,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CommitmenOptDataSliceOptEncodingMand {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub commitment: Option<Commitment>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub data_slice: Option<DataSlice>,
  pub encoding: AccountEncoding,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CommitmentOptEncoding {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub commitment: Option<Commitment>,
  pub encoding: AccountEncoding,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CommitmentOptEncodingOpt<E> {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub commitment: Option<Commitment>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub encoding: Option<E>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct DataSlice {
  pub length: usize,
  pub offset: usize,
}

// Used by `getTokenAccountsByDelegate` and `getTokenAccountsByOwner` endpoint
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum MintOrProgramId<S>
where
  S: AsRef<str>,
{
  Mint(S),
  ProgramId(S),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct JsonRpcRequestParamsWithThreeOpt<AO, BO, CO>(
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate)  Option<AO>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate)  Option<BO>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate)  Option<CO>,
);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct JsonRpcRequestParamsWithTwoMandAndOneOpt<AM, BM, CO>(
  pub(crate) AM,
  pub(crate) BM,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(crate)  Option<CO>,
);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcResponseResultContext {
  pub slot: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcResponseResultWithContext<V> {
  pub context: JsonRpcResponseResultContext,
  pub value: V,
}
