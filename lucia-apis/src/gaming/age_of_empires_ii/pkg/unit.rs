#[lucia_macros::pkg(
  api(crate::gaming::age_of_empires_ii::AgeOfEmpiresII),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::gaming::age_of_empires_ii::{AgeOfEmpiresIIHttpPackagesAux, CostResData};
  use alloc::vec::Vec;
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> AgeOfEmpiresIIHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(params: &mut UnitParams, req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/unit/{}", params.id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct UnitParams {
    id: u64,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct UnitReqData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub struct UnitResData {
    pub id: u32,
    pub name: ArrayString<21>,
    pub description: ArrayString<201>,
    pub expansion: ArrayString<14>,
    pub age: ArrayString<8>,
    pub created_in: ArrayString<74>,
    pub cost: CostResData,
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
}
