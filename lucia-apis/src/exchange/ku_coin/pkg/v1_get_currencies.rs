#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{KuCoinHttpPkgsAux, ResponseWrapper},
    misc::{MaxAddressHashStr, _MaxAssetAbbr, _MaxAssetFullName, _MaxAssetName, _MaxNumberStr},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/currencies"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1GetCurrenciesReq;

  #[pkg::res_data]
  pub type V1GetCurrenciesRes = ResponseWrapper<Vec<V1Currency>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1Currency {
    /// Number of confirmed blocks for a transaction to be considered fully valid.
    pub confirms: Option<u16>,
    /// Blockchain or network address.
    pub contract_address: Option<MaxAddressHashStr>,
    /// Immutable asset name
    pub currency: _MaxAssetAbbr,
    /// Mutable asset full name.
    pub full_name: _MaxAssetFullName,
    /// Support debit or not
    pub is_debit_enabled: bool,
    /// If asset can be deposited.
    pub is_deposit_enabled: Option<bool>,
    /// If asset supports margin trading.
    pub is_margin_enabled: bool,
    /// If asset supports withdrawal.
    pub is_withdraw_enabled: Option<bool>,
    /// Mutable asset name
    pub name: _MaxAssetName,
    /// Asset precision
    pub precision: u8,
    /// Minimum fees charged for withdrawal
    pub withdrawal_min_fee: Option<_MaxNumberStr>,
    /// Minimum withdrawal amount
    pub withdrawal_min_size: Option<_MaxNumberStr>,
  }
}
