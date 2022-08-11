use crate::gaming::age_of_empires_ii::{AgeOfEmpiresII, CivilizationRes};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = CivilizationsRes;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|json_request;

  CivilizationsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  CivilizationsParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/civilizations"))?;
    }
  }

  civilizations() {
    || {
      CivilizationsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CivilizationsRes {
  pub civilizations: Vec<CivilizationRes>,
}
