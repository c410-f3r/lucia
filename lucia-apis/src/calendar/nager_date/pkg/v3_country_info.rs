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
  async fn before_sending(
    params: &mut V3CountryInfoParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v3/CountryInfo/{}", params.country))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3CountryInfoParams<'any> {
    country: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V3CountryInfoReqData;

  #[pkg::res_data]
  pub type V3CountryInfoResData = Box<V3CountryInfoElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V3CountryInfoElemResData {
    pub common_name: ArrayString<12>,
    pub official_name: ArrayString<26>,
    pub country_code: ArrayString<12>,
    pub region: ArrayString<6>,
    pub borders: Option<Vec<V3CountryInfoElemResData>>,
  }
}
