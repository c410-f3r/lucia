use crate::{
  api::game::age_of_empires_ii::{AgeOfEmpiresII, UnitRes},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = UnitsRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  UnitsReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  UnitsParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v1/units"))?;
    }
  }

  units() {
    || {
      UnitsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct UnitsRes {
  pub units: Vec<UnitRes>,
}
