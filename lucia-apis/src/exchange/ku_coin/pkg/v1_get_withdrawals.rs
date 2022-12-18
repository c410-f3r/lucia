#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{
      Chain, HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, KuCoinId, PaginatedResponse, Status,
      WalletTxId,
    },
    misc::{into_rslt, MaxAddressHashStr, _MaxAssetAbbr, _MaxNumberStr},
  };
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetWithdrawalsParams<'_>,
    req_bytes: &[u8],
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
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
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
  pub type V1GetWithdrawalsRes = HttpResWrapper<PaginatedResponse<V1Withdrawals>>;

  /// Elements contained in the `/api/v1/withdrawals` endpoint.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1Withdrawals {
    /// Withdrawal address.
    pub address: MaxAddressHashStr,
    /// Withdrawal amount.
    pub amount: _MaxNumberStr,
    /// Blockchain or network of the asset.
    pub chain: Chain,
    /// Creation timestamp.
    pub created_at: u64,
    /// Asset identifier.
    pub currency: _MaxAssetAbbr,
    /// Withdrawal fee.
    pub fee: _MaxNumberStr,
    /// Unique identity.
    pub id: KuCoinId,
    /// Internal withdrawal or not.
    pub is_inner: bool,
    /// Address remark.
    pub memo: ArrayString<20>,
    /// Remark.
    pub remark: ArrayString<20>,
    /// Status
    pub status: Status,
    /// Update timestamp.
    pub updated_at: u64,
    /// Blockchain or network transaction id.
    pub wallet_tx_id: WalletTxId,
  }
}
