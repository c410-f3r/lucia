#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::{NagerDateHttpPackagesAux, V3PublicHolidayElemResData};
  use alloc::vec::Vec;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(
    params: &mut V3PublicHolidaysParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params
      .url
      .push_path(format_args!("/api/v3/PublicHolidays/{}/{}", params.year, params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct V3PublicHolidaysParams<'any> {
    year: i32,
    country_code: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V3PublicHolidaysReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V3PublicHolidaysResData = Vec<V3PublicHolidayElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V3PublicHolidaysElemResData {}
}
