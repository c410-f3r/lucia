/// Used by endpoints where requests are expected to have one mandatory and one optional fields.
#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub struct OneMandAndOneOpt<AM, BO>(
  /// Mandatory
  pub AM,
  /// Optional
  #[serde(skip_serializing_if = "Option::is_none")]
  pub Option<BO>,
);
