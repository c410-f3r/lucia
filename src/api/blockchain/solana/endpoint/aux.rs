use crate::api::blockchain::solana::AccountEncoding;

/// Block commitment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Commitment {
  // Middle ground
  Confirmed,
  // Most reliable
  Finalized,
  // Lesser reliable
  Processed,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentMand {
  pub commitment: Commitment,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmenOptDataSliceOptEncodingMand {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commitment: Option<Commitment>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_slice: Option<DataSlice>,
  pub encoding: AccountEncoding,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentOptEncoding {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commitment: Option<Commitment>,
  pub encoding: AccountEncoding,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentOptEncodingOpt<E> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commitment: Option<Commitment>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding: Option<E>,
}

#[derive(Debug, serde::Serialize)]
pub struct DataSlice {
  pub length: usize,
  pub offset: usize,
}

// Used by `getTokenAccountsByDelegate` and `getTokenAccountsByOwner` endpoint
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MintOrProgramId<S>
where
  S: AsRef<str>,
{
  Mint(S),
  ProgramId(S),
}

#[derive(Debug, serde::Serialize)]
pub struct JsonRpcRequestParamsWithThreeOpt<AO, BO, CO>(
  #[serde(skip_serializing_if = "Option::is_none")] pub(crate) Option<AO>,
  #[serde(skip_serializing_if = "Option::is_none")] pub(crate) Option<BO>,
  #[serde(skip_serializing_if = "Option::is_none")] pub(crate) Option<CO>,
);

#[derive(Debug, serde::Serialize)]
pub struct JsonRpcRequestParamsWithTwoMandAndOneOpt<AM, BM, CO>(
  pub(crate) AM,
  pub(crate) BM,
  #[serde(skip_serializing_if = "Option::is_none")] pub(crate) Option<CO>,
);

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcResponseResultContext {
  pub slot: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcResponseResultWithContext<V> {
  pub context: JsonRpcResponseResultContext,
  pub value: V,
}
