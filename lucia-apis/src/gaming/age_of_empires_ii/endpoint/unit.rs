use crate::gaming::age_of_empires_ii::{AgeOfEmpiresII, CostRes};
use alloc::vec::Vec;
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = UnitRes;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|json_request;

  UnitReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  UnitParams(id: u64) -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/unit/{id}"))?;
    }
  }

  unit() {
    || {
      UnitReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct UnitRes {
  pub id: u32,
  pub name: ArrayString<21>,
  pub description: ArrayString<201>,
  pub expansion: ArrayString<14>,
  pub age: ArrayString<8>,
  pub created_in: ArrayString<74>,
  pub cost: CostRes,
  pub build_time: Option<u8>,
  pub reload_time: Option<f32>,
  pub attack_delay: Option<f32>,
  pub movement_rate: Option<f32>,
  pub line_of_sight: u8,
  pub hit_points: u16,
  pub attack: Option<u8>,
  pub armor: ArrayString<5>,
  pub attack_bonus: Option<Vec<ArrayString<47>>>,
  pub armor_bonus: Option<Vec<ArrayString<20>>>,
  pub search_radius: Option<u8>,
  pub accuracy: Option<ArrayString<56>>,
  pub blast_radius: Option<f32>,
}
