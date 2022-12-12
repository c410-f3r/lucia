#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux},
    misc::into_rslt,
  };
  use arrayvec::ArrayString;
  use lucia::network::{HttpMethod, HttpReqParams};

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.orders_rt.rc.update_params(&api.orders_rt.rl).await?;
    req_params.method = HttpMethod::Post;
    req_params.url.push_path(format_args!("/api/v1/withdrawals"))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1ApplyWithdrawalReq<'any> {
    address: &'any str,
    amount: u64,
    currency: &'any str,
    #[serde(flatten)]
    options: Option<V1ApplyWithdrawalReqOpts<'any>>,
  }

  #[pkg::res_data]
  pub type V1ApplyWithdrawalRes = HttpResWrapper<V1ApplyWithdrawal>;

  /// Withdrawal fee deduction type
  #[derive(Clone, Copy, Debug, serde::Serialize)]
  pub enum FeeDeductType {
    /// Deduct the transaction fees from your withdrawal amount
    Internal,
    /// Deduct the transaction fees from your main account
    External,
  }

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1ApplyWithdrawal {
    /// Returned unique id.
    pub withdrawal_id: ArrayString<32>,
  }

  /// Additional optional request data
  #[derive(Debug, serde::Serialize)]
  pub struct V1ApplyWithdrawalReqOpts<'any> {
    /// Asset blockchain. For example, Ethereum or Solana.
    pub chain: Option<&'any str>,
    /// Fee deduct type
    pub fee_deduct_type: Option<FeeDeductType>,
    /// Internal withdrawal or not
    pub is_inner: Option<bool>,
    /// Memo
    pub memo: Option<&'any str>,
    /// Remark
    pub remark: Option<&'any str>,
  }
}
