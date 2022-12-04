#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{KuCoin, KuCoinHttpPackagesAux, ResponseWrapper},
    misc::{_MaxAssetName, _MaxNumberStr, into_rslt},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/accounts"))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1GetAccountsReq;

  #[pkg::res_data]
  pub type V1GetAccountsRes = ResponseWrapper<Vec<V1GetAccountsResElem>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1GetAccountsResElem {
    /// Funds available to withdraw or trade.
    pub available: _MaxNumberStr,
    /// Asset identifier
    pub currency: Option<_MaxAssetName>,
    /// Frozen amount.
    pub holds: _MaxNumberStr,
    /// Account type
    pub r#type: AccountTy,
  }

  /// KuCoin has three different types of accounts.
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
  #[derive(Debug)]
  pub enum AccountTy {
    /// Storage, withdrawal, and deposit of funds.
    Main,
    /// Borrow assets and leverage transactions.
    Margin,
    /// Trading of orders in the spot market.
    Trade,
  }
}
