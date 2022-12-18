#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{Chain, HttpResWrapper, KuCoin, KuCoinHttpPkgsAux},
    misc::{_MaxAssetAbbr, _MaxNumberStr, into_rslt},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetWithdrawalsQuotasParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/withdrawals/quotas"))?;
    let _ = req_params
      .url
      .query_writer()?
      .write("currency", params.currency)?
      .write_opt("chain", params.chain)?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetWithdrawalsQuotasParams<'any> {
    currency: &'any str,
    chain: Option<&'any str>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1GetWithdrawalsQuotasReq;

  #[pkg::res_data]
  pub type V1GetWithdrawalsQuotasRes = HttpResWrapper<Box<V1WithdrawalQuotas>>;

  /// Withdrawal limits associated with an asset.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1WithdrawalQuotas {
    /// Current available withdrawal amount
    pub available_amount: _MaxNumberStr,
    /// The chain name of currency, e.g. The available value for USDT are OMNI, ERC20, TRC20, default is ERC20.
    pub chain: Chain,
    /// Asset
    pub currency: _MaxAssetAbbr,
    /// Fees for internal withdrawal
    pub inner_withdraw_min_fee: _MaxNumberStr,
    /// Is the withdraw function enabled or not
    pub is_withdraw_enabled: bool,
    /// Total BTC amount available to withdraw the current day
    #[serde(rename = "limitBTCAmount")]
    pub limit_btc_amount: _MaxNumberStr,
    /// Floating point precision.
    pub precision: u8,
    /// Remaining amount available to withdraw the current day
    pub remain_amount: _MaxNumberStr,
    /// The estimated BTC amount (based on the daily fiat limit) that can be withdrawn within the current day
    #[serde(rename = "usedBTCAmount")]
    pub used_btc_amount: _MaxNumberStr,
    /// Minimum withdrawal fee
    pub withdraw_min_fee: _MaxNumberStr,
    /// Minimum withdrawal amount
    pub withdraw_min_size: _MaxNumberStr,
  }
}
