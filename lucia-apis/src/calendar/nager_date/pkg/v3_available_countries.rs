#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPackagesAux;
  use alloc::vec::Vec;
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v3/AvailableCountries"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V3AvailableCountriesReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V3AvailableCountriesResData = Vec<V3AvailableCountriesElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V3AvailableCountriesElemResData {
    pub country_code: ArrayString<2>,
    pub name: ArrayString<22>,
  }
}
