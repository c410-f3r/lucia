use crate::{api::game::age_of_empires_ii::AgeOfEmpiresII, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

type Res = CivilizationRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  CivilizationReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  CivilizationParams(id: u64) -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v1/civilization/{id}"))?;
    }
  }

  civilization() {
    || {
      CivilizationReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct CivilizationRes {
  pub army_type: ArrayString<30>,
  pub civilization_bonus: Vec<ArrayString<118>>,
  pub expansion: ArrayString<17>,
  pub id: u32,
  pub name: ArrayString<10>,
  pub team_bonus: ArrayString<93>,
  pub unique_tech: Vec<ArrayString<74>>,
  pub unique_unit: Vec<ArrayString<70>>,
}
