use crate::{api::game::age_of_empires_ii::AgeOfEmpiresII, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;
use core::fmt::Write;

type Res = UnitRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  UnitReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  unit(id: u64) -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.write_fmt(format_args!("/api/v1/unit/{id}"))?;
      UnitReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
pub struct UnitRes {
  pub id: u32,
  pub name: ArrayString<21>,
  pub description: ArrayString<201>,
  pub expansion: ArrayString<14>,
  pub age: ArrayString<8>,
  pub created_in: ArrayString<74>,
  pub cost: UnitCostRes,
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnitCostRes {
  pub wood: Option<u8>,
  pub food: Option<u8>,
  pub stone: Option<u8>,
  pub gold: Option<u8>,
}
