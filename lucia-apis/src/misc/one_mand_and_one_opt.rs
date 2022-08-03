/// Used by endpoints where requests are expected to have one mandatory and one optional fields.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct OneMandAndOneOpt<AM, BO>(
  /// Mandatory
  pub AM,
  /// Optional
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub Option<BO>,
);
