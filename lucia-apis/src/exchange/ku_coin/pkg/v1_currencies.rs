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
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/currencies"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1CurrenciesReqData;

  #[pkg::res_data]
  pub type V1CurrenciesResData = GenericDataResponse<Vec<V1CurrenciesElemResData>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1CurrenciesElemResData {
    pub confirms: Option<u16>,
    pub contract_address: Option<MaxAddressHashStr>,
    pub currency: _MaxAssetAbbr,
    pub full_name: String,
    pub is_debit_enabled: bool,
    pub is_deposit_enabled: Option<bool>,
    pub is_margin_enabled: bool,
    pub is_withdraw_enabled: Option<bool>,
    pub name: _MaxAssetName,
    pub precision: u8,
    pub withdrawal_min_fee: Option<_MaxNumberStr>,
    pub withdrawal_min_size: Option<_MaxNumberStr>,
  }
}
