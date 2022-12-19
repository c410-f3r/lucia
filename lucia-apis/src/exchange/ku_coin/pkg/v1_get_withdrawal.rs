#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, KuCoinId, Status, WalletTxId},
    misc::{into_rslt, MaxAddressHashStr, _MaxAssetAbbr, _MaxNumberStr},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetWithdrawalParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/withdrawals/{}", params.id))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetWithdrawalParams<'any> {
    id: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1GetWithdrawalReq;

  #[pkg::res_data]
  pub type V1GetWithdrawalRes = HttpResWrapper<Box<V1Withdrawal>>;

  /// Elements contained in the `/api/v1/withdrawal/SOME_WITHDRAWAL_ID` endpoint.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1Withdrawal {
    /// Withdrawal address.
    pub address: MaxAddressHashStr,
    /// Withdrawal amount.
    pub amount: _MaxNumberStr,
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
    /// Status
    pub status: Status,
    /// Update timestamp.
    pub updated_at: u64,
    /// Blockchain or network transaction id.
    pub wallet_tx_id: WalletTxId,
  }
}
