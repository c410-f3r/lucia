#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{Chain, KuCoinHttpPkgsAux, ResponseWrapper},
    misc::{MaxAddressHashStr, _MaxAssetAbbr, _MaxNumberStr},
  };
  use arrayvec::ArrayString;
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

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1GetWithdrawalsReq;

  #[pkg::res_data]
  pub type V1GetWithdrawalsRes = ResponseWrapper<Vec<V1Withdrawal>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1Withdrawal {
    /// Withdrawal address.
    pub address: MaxAddressHashStr,
    /// Withdrawal amount.
    pub amount: _MaxNumberStr,
    /// Blockchain or network of the asset.
    pub chain: Chain,
    /// Creation timestamp.
    pub created_at: i64,
    /// Asset identifier.
    pub currency: _MaxAssetAbbr,
    /// Withdrawal fee.
    pub fee: _MaxNumberStr,
    /// Unique identity.
    pub id: ArrayString<20>,
    /// Internal withdrawal or not.
    pub is_inner: bool,
    /// Address remark.
    pub memo: ArrayString<20>,
    /// Remark.
    pub remark: ArrayString<20>,
    /// Status
    pub status: ArrayString<20>,
    /// Update timestamp.
    pub updated_at: i64,
    /// Blockchain or network transaction id.
    pub wallet_tx_id: ArrayString<20>,
  }
}
