#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub struct OneMandAndOneOpt<AM, BO>(
  pub AM,
  #[serde(skip_serializing_if = "Option::is_none")] pub Option<BO>,
);
