#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::{NagerDateHttpPackagesAux, V3PublicHolidayResElem};
  use alloc::vec::Vec;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V3NextPublicHolidaysParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v3/NextPublicHolidays/{}", params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3NextPublicHolidaysParams<'any> {
    country_code: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V3NextPublicHolidaysReq;

  #[pkg::res_data]
  pub type V3NextPublicHolidaysRes = Vec<V3PublicHolidayResElem>;
}
