#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use lucia::network::{HttpReqParams, HttpResParams, StatusCode};

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::after_sending]
  async fn after_sending(res_params: &mut HttpResParams) -> crate::Result<()> {
    if res_params.status_code == StatusCode::Ok {
      Ok(())
    } else {
      Err(crate::Error::IncompatibleStatusCode(StatusCode::Ok, res_params.status_code))
    }
  }

  #[pkg::before_sending]
  async fn before_sending(
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
  #[pkg::params]
  pub struct V3IsTodayPublicHolidayParams<'any> {
    country_code: &'any str,
    county_code: Option<&'any str>,
    offset: Option<i8>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3IsTodayPublicHolidayReq;

  #[pkg::res_data]
  pub type V3IsTodayPublicHolidayRes = ();
}
