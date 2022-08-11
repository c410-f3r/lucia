use crate::gaming::age_of_empires_ii::{AgeOfEmpiresII, UnitRes};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = UnitsRes;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|json_request;

  UnitsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  UnitsParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/units"))?;
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
