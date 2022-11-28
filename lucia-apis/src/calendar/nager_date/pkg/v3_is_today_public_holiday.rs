#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPackagesAux;
  use alloc::vec::Vec;
  use arrayvec::ArrayString;
  use lucia::network::{HttpReqParams, HttpResParams, StatusCode};

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPackagesAux<DRSR> {}

  #[pkg::after_sending]
  fn after_sending(
    _: &mut V3IsTodayPublicHolidayParams<'_>,
    res_params: &mut HttpResParams,
  ) -> crate::Result<()> {
    if res_params.status_code == StatusCode::Ok {
      Ok(())
    } else {
      Err(crate::Error::IncompatibleStatusCode(StatusCode::Ok, res_params.status_code))
    }
  }

  #[pkg::before_sending]
  fn before_sending(
    params: &mut V3IsTodayPublicHolidayParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params
      .url
      .push_path(format_args!("/api/v3/IsTodayPublicHoliday/{}", params.country_code))?;
    let _ = req_params
      .url
      .query_writer()?
      .write_opt("countyCode", params.county_code)?
      .write_opt("offset", params.offset)?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct V3IsTodayPublicHolidayParams<'any> {
    country_code: &'any str,
    county_code: Option<&'any str>,
    offset: Option<i8>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V3IsTodayPublicHolidayReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V3IsTodayPublicHolidayResData = Box<V3IsTodayPublicHolidayElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V3IsTodayPublicHolidayElemResData {
    pub common_name: ArrayString<12>,
    pub official_name: ArrayString<26>,
    pub country_code: ArrayString<12>,
    pub region: ArrayString<6>,
    pub borders: Option<Vec<V3IsTodayPublicHolidayElemResData>>,
  }
}
