#[lucia_macros::pkg(
  api(crate::blockchain::aptos::Aptos),
  data_format(verbatim),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::aptos::AptosHttpPkgsAux;
  use lucia::network::{HttpReqParams, HttpResParams, StatusCode};
  use serde::de::IgnoredAny;

  #[pkg::aux]
  impl<DRSR> AptosHttpPkgsAux<DRSR> {}

  #[pkg::after_sending]
  async fn after_sending(
    api: &mut crate::blockchain::aptos::Aptos,
    res_params: &mut HttpResParams,
  ) -> crate::Result<()> {
    if res_params.status_code == StatusCode::Ok {
      api.fhrh.eval(res_params)?;
      Ok(())
    } else {
      Err(crate::Error::IncompatibleStatusCode(StatusCode::Ok, res_params.status_code))
    }
  }

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut CheckBasicNodeHealthParams,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.headers.push_str("accept", "application/json, application/x-bcs")?;
    req_params.url.push_path(format_args!("/-/healthy"))?;
    let _ = req_params.url.query_writer()?.write_opt("duration_secs", params.duration_secs)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct CheckBasicNodeHealthParams {
    duration_secs: Option<u32>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CheckBasicNodeHealthReq;

  #[pkg::res_data]
  pub type CheckBasicNodeHealthRes = IgnoredAny;
}
