#[lucia_macros::pkg(
  api(crate::gaming::age_of_empires_ii::AgeOfEmpiresII),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::gaming::age_of_empires_ii::AgeOfEmpiresIIHttpPackagesAux;
  use alloc::vec::Vec;
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> AgeOfEmpiresIIHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(
    params: &mut CivilizationParams,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/civilization/{}", params.id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct CivilizationParams {
    id: u64,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct CivilizationReqData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub struct CivilizationResData {
    pub army_type: ArrayString<30>,
    pub civilization_bonus: Vec<ArrayString<118>>,
    pub expansion: ArrayString<17>,
    pub id: u32,
    pub name: ArrayString<10>,
    pub team_bonus: ArrayString<93>,
    pub unique_tech: Vec<ArrayString<74>>,
    pub unique_unit: Vec<ArrayString<70>>,
  }
}
