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
  fn before_sending(
    params: &mut TechnologyParams,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/technology/{}", params.id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct TechnologyParams {
    id: u64,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct TechnologyReqData;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub struct TechnologyResData {
    pub id: u32,
    pub name: ArrayString<21>,
    pub description: ArrayString<201>,
    pub expansion: ArrayString<14>,
    pub age: ArrayString<8>,
    pub develops_in: ArrayString<74>,
    pub cost: CostResData,
    pub build_time: Option<u8>,
    pub applies_to: Option<Vec<ArrayString<73>>>,
  }
}
