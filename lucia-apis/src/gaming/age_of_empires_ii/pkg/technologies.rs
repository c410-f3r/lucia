#[lucia_macros::pkg(
  api(crate::gaming::age_of_empires_ii::AgeOfEmpiresII),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::gaming::age_of_empires_ii::{AgeOfEmpiresIIHttpPackagesAux, TechnologyResData};
  use alloc::vec::Vec;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> AgeOfEmpiresIIHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/technologies"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct TechnologiesReqData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub struct TechnologiesResData {
    pub technologies: Vec<TechnologyResData>,
  }
}
