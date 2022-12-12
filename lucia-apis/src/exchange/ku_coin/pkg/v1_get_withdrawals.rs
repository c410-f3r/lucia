#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::exchange::ku_coin::{HttpResWrapper, KuCoinHttpPkgsAux, V1Withdrawal};
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V1GetWithdrawalsParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/withdrawals"))?;
    let _ = req_params
      .url
      .query_writer()?
      .write_opt("currency", params.currency)?
      .write_opt("status", params.status)?
      .write_opt("startAt", params.start_at)?
      .write_opt("endAt", params.end_at)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetWithdrawalsParams<'any> {
    currency: Option<&'any str>,
    end_at: Option<u64>,
    start_at: Option<u64>,
    status: Option<&'any str>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1GetWithdrawalsReq;

  #[pkg::res_data]
  pub type V1GetWithdrawalsRes = HttpResWrapper<Vec<V1Withdrawal>>;
}
