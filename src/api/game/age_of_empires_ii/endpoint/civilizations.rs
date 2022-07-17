use crate::{
  api::game::age_of_empires_ii::{AgeOfEmpiresII, CivilizationRes},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = CivilizationsRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  CivilizationsReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  CivilizationsParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v1/civilizations"))?;
    }
  }

  civilizations() {
    || {
      CivilizationsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct CivilizationsRes {
  pub civilizations: Vec<CivilizationRes>,
}
