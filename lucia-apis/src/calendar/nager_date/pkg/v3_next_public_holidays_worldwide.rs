#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::{NagerDateHttpPkgsAux, V3PublicHoliday};
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v3/NextPublicHolidaysWorldwide"))?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3NextPublicHolidaysWorldwideReq;

  #[pkg::res_data]
  pub type V3NextPublicHolidaysWorldwideRes = Vec<V3PublicHoliday>;
}
