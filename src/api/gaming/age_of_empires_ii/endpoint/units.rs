use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, UnitRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

type Res = UnitsRes;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  UnitsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  UnitsParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/units"))?;
    }
  }

  units() {
    || {
      UnitsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct UnitsRes {
  pub units: Vec<UnitRes>,
}
