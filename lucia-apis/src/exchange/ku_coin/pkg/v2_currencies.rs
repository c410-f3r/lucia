#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{GenericDataResponse, KuCoinHttpPackagesAux},
    misc::{MaxAddressHashStr, _MaxAssetAbbr, _MaxAssetName, _MaxNumberStr},
  };
  use arrayvec::ArrayVec;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(
    params: &mut V2CurrenciesParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v2/currencies/{}", params.asset))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct V2CurrenciesParams<'any> {
    asset: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V2CurrenciesReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V2CurrenciesResData = GenericDataResponse<Box<V2CurrenciesElemResData>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V2CurrenciesElemResData {
    pub chains: ArrayVec<V2CurrenciesChainResData, 4>,
    pub currency: _MaxAssetAbbr,
    pub name: _MaxAssetName,
    pub full_name: _MaxAssetName,
    pub precision: u8,
    pub confirms: Option<u16>,
    pub contract_address: Option<MaxAddressHashStr>,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V2CurrenciesChainResData {
    pub chain_name: Option<_MaxAssetName>,
    pub confirms: u16,
    pub contract_address: Option<MaxAddressHashStr>,
    pub is_deposit_enabled: bool,
    pub is_withdraw_enabled: bool,
    pub withdrawal_min_fee: _MaxNumberStr,
    pub withdrawal_min_size: _MaxNumberStr,
  }
}
