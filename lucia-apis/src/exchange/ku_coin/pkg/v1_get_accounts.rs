#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, V1Account, V1AccountTy},
    misc::into_rslt,
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetAccountsParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/accounts"))?;
    let _ = req_params
      .url
      .query_writer()?
      .write_opt("currency", params.currency)?
      .write_opt("type", params.r#type)?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetAccountsParams<'any> {
    currency: Option<&'any str>,
    r#type: Option<V1AccountTy>,
  }

  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  #[pkg::req_data]
  pub struct V1GetAccountsReq;

  #[pkg::res_data]
  pub type V1GetAccountsRes = HttpResWrapper<Vec<V1Account>>;
}
