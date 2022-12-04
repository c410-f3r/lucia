#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{KuCoinHttpPackagesAux, ResponseWrapper},
    misc::{MaxAddressHashStr, _MaxAssetAbbr, _MaxAssetName, _MaxNumberStr},
  };
  use arrayvec::ArrayVec;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V2GetCurrenciesParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v2/currencies/{}", params.currency))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V2GetCurrenciesParams<'any> {
    currency: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V2GetCurrenciesReq;

  #[pkg::res_data]
  pub type V2GetCurrenciesRes = ResponseWrapper<Box<V2GetCurrenciesResElem>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V2GetCurrenciesResElem {
    /// Blockchains or networks
    pub chains: ArrayVec<V2GetCurrenciesChain, 4>,
    /// Immutable asset name
    pub currency: _MaxAssetAbbr,
    /// Mutable asset name
    pub name: _MaxAssetName,
    /// Mutable full asset name
    pub full_name: _MaxAssetName,
    /// Decimal precision
    pub precision: u8,
    /// Number of confirmed blocks for a transaction to be considered fully valid.
    pub confirms: Option<u16>,
    /// Hash address
    pub contract_address: Option<MaxAddressHashStr>,
    /// If asset supports margin trading.
    pub is_margin_enabled: bool,
    /// Support debit or not
    pub is_debit_enabled: bool,
  }

  /// Blockchain or network information.
  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  pub struct V2GetCurrenciesChain {
    /// Blockchain name
    pub chain_name: Option<_MaxAssetName>,
    /// Number of confirmed blocks for a transaction to be considered fully valid.
    pub confirms: u16,
    /// Hash address
    pub contract_address: Option<MaxAddressHashStr>,
    /// If asset can be deposited in the network.
    pub is_deposit_enabled: bool,
    /// If asset can be transferred in the network.
    pub is_withdraw_enabled: bool,
    /// Minimum fee charged by the network
    pub withdrawal_min_fee: _MaxNumberStr,
    /// Minimum allowed withdrawl in the network;
    pub withdrawal_min_size: _MaxNumberStr,
  }
}
